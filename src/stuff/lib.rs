// the "link" crate attribute is currently required for rustdoc, but normally
// isn't needed.
#![crate_id = "universe"]
#![crate_type="lib"]

use std::string::String;

/// Widgets are very common (this is a doc comment, and will show up on
/// Widget's documentation).
pub struct Widget {
    /// All widgets have a purpose (this is a doc comment, and will show up
    /// the field's documentation).
    purpose: String,
    /// Humans are not allowed to understand some widgets
    understandable: bool
}

pub fn recalibrate() {
    //! Recalibrate a pesky universe (this is also a doc comment, like above,
    //! the documentation will be applied to the *parent* item, so
    //! `recalibrate`).
    /* ... */
}

