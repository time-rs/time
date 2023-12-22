#[cfg(feature = "parsing")]
use crate::error;
use crate::internal_macros::bug;
#[cfg(feature = "parsing")]
use crate::parsing::Parsed;
use crate::UtcOffset;

pub(crate) type MemoryOffsetType<T> = <T as MaybeTz>::MemoryOffset;
pub(crate) type LogicalOffsetType<T> = <T as MaybeTz>::LogicalOffset;
pub(crate) type TzType<T> = <T as MaybeTz>::Tz;

pub(crate) mod offset_kind {
    #[derive(Debug, Clone, Copy)]
    pub enum None {}
    #[derive(Debug, Clone, Copy)]
    pub enum Fixed {}
}

#[derive(Debug, Clone, Copy)]
pub struct NoOffset;

#[derive(Debug, Clone, Copy)]
pub struct NoTz;

/// A type that is guaranteed to be either [`NoOffset`] or [`UtcOffset`].
///
/// # Safety
///
/// This strait may only be implemented for [`NoOffset`] and [`UtcOffset`].
pub unsafe trait MaybeOffsetType: Copy {}
// Safety: The trait is permitted to be implemented for this type.
unsafe impl MaybeOffsetType for NoOffset {}
// Safety: The trait is permitted to be implemented for this type.
unsafe impl MaybeOffsetType for UtcOffset {}

/// A type that is guaranteed to be either [`NoTz`] or a type that implements `TimeZone`.
///
/// # Safety
///
/// This trait may only be implemented for [`NoTz`] and types that implement `TimeZone`.
pub unsafe trait MaybeTzType {}
// Safety: The trait is permitted to be implemented for this type.
unsafe impl MaybeTzType for NoTz {}
// TODO Add blanket implementation for all time zones here.

/// # Safety
///
/// - The associated type `Self_` must be `Self`.
/// - The associated const `HAS_MEMORY_OFFSET` must be `true` if and only if the associated type
///   `MemoryOffset` is `UtcOffset`.
/// - The associated const `HAS_LOGICAL_OFFSET` must be `true` if and only if the associated type
///   `LogicalOffset` is `UtcOffset`.
/// - The associated const `HAS_TZ` must be `true` if and only if the associated type `Tz`
///   implements `TimeZone`.
pub unsafe trait MaybeTz {
    /// The offset type as it is stored in memory.
    type MemoryOffset: MaybeOffsetType;
    /// The offset type as it should be thought about.
    ///
    /// For example, a `DateTime<Utc>` would have a logical offset type of [`UtcOffset`], but does
    /// not actually store an offset in memory.
    type LogicalOffset: MaybeOffsetType;
    /// The type of the time zone, which is used to calculate the UTC offset at a given point in
    /// time.
    type Tz: MaybeTzType;
    /// Required to be `Self`. Used for bound equality.
    type Self_;

    /// True if and only if `Self::Memory` is `UtcOffset`.
    const HAS_MEMORY_OFFSET: bool;
    /// True if and only if `Self::Logical` is `UtcOffset`.
    const HAS_LOGICAL_OFFSET: bool;
    /// True if and only if `Self::Tz` implements `TimeZone`.
    const HAS_TZ: bool;
    /// `Some` if and only if the logical UTC offset is statically known.
    // TODO(jhpratt) When const trait impls are stable, this can be removed in favor of
    // `.as_offset_opt()`.
    const STATIC_OFFSET: Option<UtcOffset>;

    fn offset_memory_to_logical_opt(offset: MemoryOffsetType<Self>) -> Option<UtcOffset>;
    fn offset_memory_to_logical(offset: MemoryOffsetType<Self>) -> UtcOffset {
        match Self::offset_memory_to_logical_opt(offset) {
            Some(offset) => offset,
            None => bug!("MaybeOffset::as_offset` called on a type without an offset in memory"),
        }
    }
    fn offset_logical_to_memory(offset: UtcOffset) -> MemoryOffsetType<Self>;
    fn as_tz(&self) -> &TzType<Self>;

    #[cfg(feature = "parsing")]
    fn try_from_parsed(parsed: Parsed) -> Result<MemoryOffsetType<Self>, error::TryFromParsed>;
}

// Safety: All requirements are upheld.
unsafe impl MaybeTz for offset_kind::None {
    type MemoryOffset = NoOffset;
    type LogicalOffset = NoOffset;
    type Tz = NoTz;

    type Self_ = Self;

    const HAS_MEMORY_OFFSET: bool = false;
    const HAS_LOGICAL_OFFSET: bool = false;
    const HAS_TZ: bool = false;
    const STATIC_OFFSET: Option<UtcOffset> = None;

    fn offset_memory_to_logical_opt(_: MemoryOffsetType<Self>) -> Option<UtcOffset> {
        None
    }

    fn offset_logical_to_memory(_: UtcOffset) -> MemoryOffsetType<Self> {
        NoOffset
    }

    fn as_tz(&self) -> &TzType<Self> {
        &NoTz
    }

    #[cfg(feature = "parsing")]
    fn try_from_parsed(_: Parsed) -> Result<MemoryOffsetType<Self>, error::TryFromParsed> {
        Ok(NoOffset)
    }
}

// Safety: All requirements are upheld.
unsafe impl MaybeTz for offset_kind::Fixed {
    type MemoryOffset = UtcOffset;
    type LogicalOffset = UtcOffset;
    type Tz = NoTz;

    type Self_ = Self;

    const HAS_MEMORY_OFFSET: bool = true;
    const HAS_LOGICAL_OFFSET: bool = true;
    const HAS_TZ: bool = false;
    const STATIC_OFFSET: Option<UtcOffset> = None;

    fn offset_memory_to_logical_opt(offset: MemoryOffsetType<Self>) -> Option<UtcOffset> {
        Some(offset)
    }

    fn offset_logical_to_memory(offset: UtcOffset) -> MemoryOffsetType<Self> {
        offset
    }

    fn as_tz(&self) -> &TzType<Self> {
        &NoTz
    }

    #[cfg(feature = "parsing")]
    fn try_from_parsed(parsed: Parsed) -> Result<MemoryOffsetType<Self>, error::TryFromParsed> {
        parsed.try_into()
    }
}

// region: const trait method hacks
// TODO(jhpratt) When const trait impls are stable, these methods can be removed in favor of the
// methods in `MaybeOffset`, which would then be made `const`.
pub(crate) const fn offset_memory_to_logical_opt<T: MaybeTz>(
    offset: MemoryOffsetType<T>,
) -> Option<UtcOffset> {
    if T::STATIC_OFFSET.is_some() {
        T::STATIC_OFFSET
    } else if T::HAS_MEMORY_OFFSET {
        #[repr(C)] // needed to guarantee they align at the start
        union Convert<T: MaybeTz> {
            input: MemoryOffsetType<T>,
            output: UtcOffset,
        }

        // Safety: `T::HAS_OFFSET` indicates that `T::Offset` is `UtcOffset`. This code effectively
        // performs a transmute from `T::Offset` to `UtcOffset`, which we know is the same type.
        Some(unsafe { Convert::<T> { input: offset }.output })
    } else {
        None
    }
}

pub(crate) const fn offset_memory_to_logical<T: MaybeTz + HasLogicalOffset>(
    offset: MemoryOffsetType<T>,
) -> LogicalOffsetType<T> {
    match offset_memory_to_logical_opt::<T>(offset) {
        Some(offset) => offset,
        // Safety: `T` is bound by `HasLogicalOffset`.
        None => unsafe { core::hint::unreachable_unchecked() },
    }
}

// TODO Add `HasLogicalOffset` bound to `T` if possible.
pub(crate) const fn offset_logical_to_memory<T: MaybeTz>(offset: UtcOffset) -> MemoryOffsetType<T> {
    #[repr(C)] // needed to guarantee the types align at the start
    union Convert<T: MaybeTz> {
        input: UtcOffset,
        output: MemoryOffsetType<T>,
    }

    // Safety: It is statically known that there are only two possibilities due to the trait bound
    // of `T::MemoryOffsetType`, which ultimately relies on `MaybeOffsetType`. The two possibilities
    // are:
    //   1. UtcOffset -> UtcOffset
    //   2. UtcOffset -> NoOffset
    // (1) is valid because it is an identity conversion, which is always valid. (2) is valid
    // because `NoOffset` is a 1-ZST, so converting to it is always valid.
    unsafe { Convert::<T> { input: offset }.output }
}
// endregion const trait method hacks

// region: marker traits
// Note: All traits in this region may be relied upon for soundness. The traits are not unsafe
// because the supertrait is unsafe.

pub trait HasLogicalOffset: MaybeTz<LogicalOffset = UtcOffset> {}
impl<T: MaybeTz<LogicalOffset = UtcOffset>> HasLogicalOffset for T {}

pub trait SansLogicalOffset: MaybeTz<LogicalOffset = NoOffset> + SansTz {}
impl<T: MaybeTz<LogicalOffset = NoOffset> + SansTz> SansLogicalOffset for T {}

pub trait HasMemoryOffset: MaybeTz<MemoryOffset = UtcOffset> {}
impl<T: MaybeTz<MemoryOffset = UtcOffset>> HasMemoryOffset for T {}

pub trait SansMemoryOffset: MaybeTz<MemoryOffset = NoOffset> {}
impl<T: MaybeTz<MemoryOffset = NoOffset>> SansMemoryOffset for T {}

// TODO Add `HasTz` trait here.

pub trait SansTz: MaybeTz<Tz = NoTz> {}
impl<T: MaybeTz<Tz = NoTz>> SansTz for T {}

pub trait IsOffsetKindNone:
    MaybeTz<Self_ = offset_kind::None> + SansLogicalOffset + SansMemoryOffset + SansTz
{
}
impl IsOffsetKindNone for offset_kind::None {}

pub trait IsOffsetKindFixed:
    MaybeTz<Self_ = offset_kind::Fixed> + HasLogicalOffset + HasMemoryOffset + SansTz
{
}
impl IsOffsetKindFixed for offset_kind::Fixed {}
// endregion marker traits
