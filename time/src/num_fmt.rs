//! Formatting utilities for numbers.
//!
//! These functions are low-level, but are designed to be _extremely_ fast for their designed use
//! cases. They are `unsafe` to avoid validating input, have strict requirements, and may not return
//! the most ergonomic types to avoid unnecessary allocations and copying.

use core::{hint, slice};

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
