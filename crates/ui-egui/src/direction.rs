//! The interface's reading direction: right to left in Arabic (and future Hebrew), left to right
//! everywhere else.
//!
//! egui's own widgets follow [`egui::Style::right_to_left`] (`theme::apply_visuals` and
//! `FilmcraftApp::apply_prefs` set it), which mirrors menus, scroll bars, headers, combo boxes,
//! checkboxes, popups and resize grips. The rows this crate lays out by hand go through [`row`]
//! and [`from_end`] instead of `Layout::left_to_right` / `Layout::right_to_left`, so they follow
//! the language too.

use egui::{Align, Layout};

/// Whether the interface is drawn right to left (see [`crate::i18n::Language::is_rtl`]).
pub(crate) fn is_rtl() -> bool {
    crate::i18n::current().is_rtl()
}

/// A horizontal row that reads in the interface's direction: items begin at the leading edge
/// (left in a left-to-right interface, right in a right-to-left one).
pub(crate) fn row(align: Align) -> Layout {
    if is_rtl() { Layout::right_to_left(align) } else { Layout::left_to_right(align) }
}

/// A horizontal row anchored to the trailing edge, the mirror of [`row`]: items begin at the
/// right in a left-to-right interface and at the left in a right-to-left one (a right-aligned
/// label column, a dialog's OK/Cancel cluster).
pub(crate) fn from_end(align: Align) -> Layout {
    if is_rtl() { Layout::left_to_right(align) } else { Layout::right_to_left(align) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rows_follow_the_language_and_come_back() {
        crate::i18n::set_current(crate::i18n::Language::En);
        assert!(!is_rtl());
        assert_eq!(row(Align::Center).main_dir, egui::Direction::LeftToRight);
        assert_eq!(from_end(Align::Center).main_dir, egui::Direction::RightToLeft);
        crate::i18n::set_current(crate::i18n::Language::Ar);
        assert!(is_rtl());
        assert_eq!(row(Align::Center).main_dir, egui::Direction::RightToLeft);
        assert_eq!(from_end(Align::Center).main_dir, egui::Direction::LeftToRight);
        crate::i18n::set_current(crate::i18n::Language::En);
    }
}
