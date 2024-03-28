//! This file is intended to be used as a stupidity-check when it comes
//! to API compatibility. If you've broken something here, you've broken
//! the public API.

// TODO: This should be completed with all methods from the `Theme` trait.

use crate::{theme::ClackTheme, Theme};

#[test]
fn format_note() {
    ClackTheme.format_note("my prompt", "my message");
}