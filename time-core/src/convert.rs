//! Conversion between units of time.

/// Declare the structs that represent a unit of time.
macro_rules! declare_structs {
    ($($t:ident $str:expr)*) => {$(
        #[doc = concat!("A unit of time representing exactly one ", $str, ".")]
        #[derive(Debug, Copy, Clone)]
        pub struct $t;

        impl $t {
            /// Obtain the integer ratio between the two units of time.
            ///
            /// If the ratio is less than one, the call will fail to compile.
            pub const fn per<T>(self, _: T) -> <(Self, T) as Per>::Output
            where
                (Self, T): Per,
                T: Copy,
            {
                <(Self, T)>::VALUE
            }
        }
    )*};
}

declare_structs! {
    Nanosecond "nanosecond"
    Microsecond "microsecond"
    Millisecond "millisecond"
    Second "second"
    Minute "minute"
    Hour "hour"
    Day "day"
    Week "week"
}

mod sealed {
    pub trait Sealed {}
}

/// A trait for defining the ratio of two units of time.
///
/// This trait is used to implement the `per` method on the various structs.
pub trait Per: sealed::Sealed {
    /// The smallest unsigned integer type that can represent [`VALUE`](Self::VALUE).
    type Output;

    /// The number of one unit of time in the other.
    const VALUE: Self::Output;
}

/// Implement the `Per` trait for pairs of types.
macro_rules! impl_per {
    ($($t:ty : $x:ident in $y:ident = $val:expr)*) => {$(
        impl sealed::Sealed for ($x, $y) {}

        impl Per for ($x, $y) {
            type Output = $t;

            const VALUE: $t = $val;
        }
    )*};
}

impl_per! {
    u8: Nanosecond in Nanosecond = 1
    u16: Nanosecond in Microsecond = 1_000
    u32: Nanosecond in Millisecond = 1_000_000
    u32: Nanosecond in Second = 1_000_000_000
    u64: Nanosecond in Minute = 60_000_000_000
    u64: Nanosecond in Hour = 3_600_000_000_000
    u64: Nanosecond in Day = 86_400_000_000_000
    u64: Nanosecond in Week = 604_800_000_000_000

    u8: Microsecond in Microsecond = 1
    u16: Microsecond in Millisecond = 1_000
    u32: Microsecond in Second = 1_000_000
    u32: Microsecond in Minute = 60_000_000
    u32: Microsecond in Hour = 3_600_000_000
    u64: Microsecond in Day = 86_400_000_000
    u64: Microsecond in Week = 604_800_000_000

    u8: Millisecond in Millisecond = 1
    u16: Millisecond in Second = 1_000
    u16: Millisecond in Minute = 60_000
    u32: Millisecond in Hour = 3_600_000
    u32: Millisecond in Day = 86_400_000
    u32: Millisecond in Week = 604_800_000

    u8: Second in Second = 1
    u8: Second in Minute = 60
    u16: Second in Hour = 3_600
    u32: Second in Day = 86_400
    u32: Second in Week = 604_800

    u8: Minute in Minute = 1
    u8: Minute in Hour = 60
    u16: Minute in Day = 1_440
    u16: Minute in Week = 10_080

    u8: Hour in Hour = 1
    u8: Hour in Day = 24
    u8: Hour in Week = 168

    u8: Day in Day = 1
    u8: Day in Week = 7
}
