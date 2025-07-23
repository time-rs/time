//! A fallback for any OS not covered.

#[expect(clippy::missing_docs_in_private_items)]
pub(super) unsafe fn refresh_tz_unchecked() {}

#[expect(clippy::missing_docs_in_private_items)]
pub(super) fn refresh_tz() -> Option<()> {
    Some(())
}
