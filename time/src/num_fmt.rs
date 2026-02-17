//! Formatting utilities for numbers.
//!
//! These functions are low-level, but are designed to be _extremely_ fast for their designed use
//! cases. They are `unsafe` to avoid validating input, have strict requirements, and may not return
//! the most ergonomic types to avoid unnecessary allocations and copying.

use core::mem::MaybeUninit;
use core::ops::Deref;
use core::{hint, slice};

static SINGLE_DIGITS: [u8; 10] = *b"0123456789";

static ZERO_PADDED_PAIRS: [u8; 200] = *b"0001020304050607080910111213141516171819\
                                         2021222324252627282930313233343536373839\
                                         4041424344454647484950515253545556575859\
                                         6061626364656667686970717273747576777879\
                                         8081828384858687888990919293949596979899";

static SPACE_PADDED_PAIRS: [u8; 200] = *b" 0 1 2 3 4 5 6 7 8 910111213141516171819\
                                          2021222324252627282930313233343536373839\
                                          4041424344454647484950515253545556575859\
                                          6061626364656667686970717273747576777879\
                                          8081828384858687888990919293949596979899";

/// A string type with a maximum length known at compile time, stored on the stack.
///
/// Note that while the _maximum_ length is known at compile time, the string may be shorter. This
/// information is stored inline.
#[derive(Clone, Copy)]
pub(crate) struct StackStr<const MAX_LEN: usize> {
    buf: [MaybeUninit<u8>; MAX_LEN],
    len: usize,
}

impl<const MAX_LEN: usize> StackStr<MAX_LEN> {
    /// # Safety:
    ///
    /// - `buf` must be initialized for at least `len` bytes.
    /// - The first `len` bytes of `buf` must be valid UTF-8.
    #[inline]
    pub(crate) const unsafe fn new(buf: [MaybeUninit<u8>; MAX_LEN], len: usize) -> Self {
        debug_assert!(len <= MAX_LEN);
        Self { buf, len }
    }
}

impl<const MAX_LEN: usize> Deref for StackStr<MAX_LEN> {
    type Target = str;

    #[inline]
    fn deref(&self) -> &Self::Target {
        // Safety: This type can only be constructed when the caller asserts that the buffer is
        // valid UTF-8 for the first `len` bytes.
        unsafe { str_from_raw_parts(self.buf.as_ptr().cast(), self.len) }
    }
}

/// Safety:
///
/// - `ptr` must be non-null and point to `len` initialized bytes of UTF-8 data.
/// - `ptr` is valid for (and not mutated during) lifetime `'a`.
#[inline]
pub(crate) const unsafe fn str_from_raw_parts<'a>(ptr: *const u8, len: usize) -> &'a str {
    // Safety: The caller must ensure that `ptr` is valid for `len` bytes and that the bytes are
    // valid UTF-8. The caller must also ensure that the lifetime `'a` is valid for the returned
    // string.
    unsafe { str::from_utf8_unchecked(slice::from_raw_parts(ptr, len)) }
}

/// Obtain a string containing a single ASCII digit representing `n`.
///
/// # Safety: `n` must be less than 10.
#[inline]
pub(crate) const unsafe fn single_digit_no_padding(n: u8) -> &'static str {
    debug_assert!(n < 10);

    // Safety: We're staying within the bounds of the array. The array contains only ASCII
    // characters, so it's valid UTF-8.
    unsafe { str_from_raw_parts(SINGLE_DIGITS.as_ptr().add(n as usize), 1) }
}

/// Obtain a string of one or two ASCII digits representing `n`. No leading zeros or spaces are
/// included.
///
/// # Safety: `n` must be less than 100.
#[inline]
pub(crate) const unsafe fn under_100_no_padding(n: u8) -> &'static str {
    debug_assert!(n < 100);

    let is_single_digit = n < 10;
    // Safety: We're staying within the bounds of the array. The array contains only ASCII
    // characters, so it's valid UTF-8.
    unsafe {
        str_from_raw_parts(
            ZERO_PADDED_PAIRS
                .as_ptr()
                .add((n as usize) * 2 + is_single_digit as usize),
            2 - is_single_digit as usize,
        )
    }
}

/// Obtain a string of two ASCII digits representing `n`. This includes a leading zero if `n` is
/// less than 10.
///
/// # Safety: `n` must be less than 100.
#[inline]
pub(crate) const unsafe fn two_digits_zero_padded(n: u8) -> &'static str {
    debug_assert!(n < 100);

    // Safety: We're staying within the bounds of the array. The array contains only ASCII
    // characters, so it's valid UTF-8.
    unsafe { str_from_raw_parts(ZERO_PADDED_PAIRS.as_ptr().add((n as usize) * 2), 2) }
}

/// Obtain a string of two ASCII digits representing `n`. This includes a leading space if `n` is
/// less than 10.
///
/// # Safety: `n` must be less than 100.
#[expect(dead_code, reason = "likely to be used in the future")]
#[inline]
pub(crate) const unsafe fn two_digits_space_padded(n: u8) -> &'static str {
    debug_assert!(n < 100);

    // Safety: We're staying within the bounds of the array. The array contains only ASCII
    // characters, so it's valid UTF-8.
    unsafe { str_from_raw_parts(SPACE_PADDED_PAIRS.as_ptr().add((n as usize) * 2), 2) }
}

/// Obtain two strings of two ASCII digits each representing `n`. The first string is the most
/// significant. Leading zeros are included if the number has fewer than 4 digits.
///
/// # Safety: `n` must be less than 10,000.
#[inline]
pub(crate) const unsafe fn four_digits(n: u16) -> [&'static str; 2] {
    debug_assert!(n < 10_000);

    const EXP: u32 = 19; // 19 is faster or equal to 12 even for 3 digits.
    const SIG: u32 = (1 << EXP) / 100 + 1;

    let high = (n as u32 * SIG) >> EXP; // value / 100
    let low = n as u32 - high * 100;

    // Safety: We're staying within the bounds of the array.
    unsafe {
        [
            two_digits_zero_padded(high as u8),
            two_digits_zero_padded(low as u8),
        ]
    }
}

/// Obtain three strings which together represent `n`. The first string is the most significant.
/// Leading zeros are included if the number has fewer than 4 digits. The first string will be empty
/// if `n` is less than 10,000.
///
/// # Safety: `n` must be less than 1,000,000.
#[inline]
pub(crate) const unsafe fn four_to_six_digits(n: u32) -> [&'static str; 3] {
    debug_assert!(n < 1_000_000);
    // Safety: The caller must ensure that this is true.
    unsafe { hint::assert_unchecked(n < 1_000_000) };

    let (first_two, remaining) = (n / 10_000, n % 10_000);

    let size = 2 - (first_two < 10) as usize - (first_two == 0) as usize;
    let offset = first_two as usize * 2 + 2 - size;

    // Safety: `offset` is within the bounds of the array. The array contains only ASCII characters,
    // so it's valid UTF-8.
    let first_two = unsafe { str_from_raw_parts(ZERO_PADDED_PAIRS.as_ptr().add(offset), size) };
    // Safety: `remaining` is guaranteed to be less than 10,000 due to the modulus above.
    let [second_two, last_two] = unsafe { four_digits(remaining as u16) };
    [first_two, second_two, last_two]
}

/// Obtain five strings which together represent `n`, which is a number of nanoseconds.
///
/// This value is intended to be used after a decimal point to represent a fractional second. The
/// first string will always contain exactly one digit; the remaining four will contain two digits
/// each.
///
/// # Safety: `n` must be less than 1,000,000,000.
#[inline]
pub(crate) const unsafe fn subsecond_from_nanos(n: u32) -> [&'static str; 5] {
    let (digits_1_thru_5, digits_6_thru_9) = (n / 10_000, n % 10_000);
    let (digit_1, digits_2_thru_5) = (digits_1_thru_5 / 10_000, digits_1_thru_5 % 10_000);

    // Safety: The caller must ensure that `n` is less than 1,000,000,000. Combined with the
    // arithmetic above, this guarantees that all values are in the required ranges.
    unsafe {
        let digit_1 = single_digit_no_padding(digit_1 as u8);
        let [digits_2_and_3, digits_4_and_5] = four_digits(digits_2_thru_5 as u16);
        let [digits_6_and_7, digits_8_and_9] = four_digits(digits_6_thru_9 as u16);

        [
            digit_1,
            digits_2_and_3,
            digits_4_and_5,
            digits_6_and_7,
            digits_8_and_9,
        ]
    }
}

/// Obtain a string of 1 to 9 ASCII digits representing `n`, which is a number of nanoseconds.
///
/// This value is intended to be used after a decimal point to represent a fractional second.
/// Trailing zeros are truncated, but at least one digit is always present.
///
/// # Safety: `n` must be less than 1,000,000,000.
#[inline]
pub(crate) const unsafe fn truncated_subsecond_from_nanos(n: u32) -> StackStr<9> {
    #[repr(C, align(8))]
    #[derive(Clone, Copy)]
    struct Digits {
        _padding: MaybeUninit<[u8; 7]>,
        digit_1: u8,
        digits_2_thru_9: [u8; 8],
    }

    // Safety: The caller must ensure that `n` is less than 1,000,000,000.
    let [
        digit_1,
        digits_2_and_3,
        digits_4_and_5,
        digits_6_and_7,
        digits_8_and_9,
    ] = unsafe { subsecond_from_nanos(n) };

    // Ensure that digits 2 thru 9 are stored as a single array that is 8-aligned. This allows the
    // conversion to a `u64` to be zero cost, resulting in a nontrivial performance improvement.
    let buf = Digits {
        _padding: MaybeUninit::uninit(),
        digit_1: digit_1.as_bytes()[0],
        digits_2_thru_9: [
            digits_2_and_3.as_bytes()[0],
            digits_2_and_3.as_bytes()[1],
            digits_4_and_5.as_bytes()[0],
            digits_4_and_5.as_bytes()[1],
            digits_6_and_7.as_bytes()[0],
            digits_6_and_7.as_bytes()[1],
            digits_8_and_9.as_bytes()[0],
            digits_8_and_9.as_bytes()[1],
        ],
    };

    // By converting the bytes into a single integer, we can effectively perform an equality check
    // against b'0' for all bytes at once. This is actually faster than using portable SIMD (even
    // with `-Ctarget-cpu=native`).
    let bitmask = u64::from_le_bytes(buf.digits_2_thru_9) ^ u64::from_le_bytes([b'0'; 8]);
    let digits_to_truncate = bitmask.leading_zeros() / 8;
    let len = 9 - digits_to_truncate as usize;

    // Safety: All bytes are initialized and valid UTF-8, and `len` represents the number of bytes
    // we wish to display (that is between 1 and 9 inclusive). `Digits` is `#[repr(C)]`, so the
    // layout is guaranteed.
    unsafe {
        StackStr::new(
            *(&raw const buf)
                .byte_add(core::mem::offset_of!(Digits, digit_1))
                .cast(),
            len,
        )
    }
}
