//! Hints to the compiler that affects how code should be emitted or optimized.

#![allow(dead_code)] // may be used in the future and has minimal overhead

/// Indicate that a given branch is **not** likely to be taken, relatively speaking.
#[inline(always)]
#[cold]
pub(crate) const fn cold_path() {}

/// Indicate that a given condition is likely to be true.
#[inline(always)]
pub(crate) const fn likely(b: bool) -> bool {
    if !b {
        cold_path();
    }
    b
}

/// Indicate that a given condition is likely to be false.
#[inline(always)]
pub(crate) const fn unlikely(b: bool) -> bool {
    if b {
        cold_path();
    }
    b
}

/// Indicate that the provided boolean condition is always true and may be relied upon for safety.
#[track_caller]
#[inline(always)]
pub(crate) const unsafe fn assert_unchecked(b: bool) {
    debug_assert!(b);
    if !b {
        // Safety: The caller has asserted that this condition is always true.
        unsafe { core::hint::unreachable_unchecked() }
    }
}
