//! Common layout builder for protocol options panels
//!
//! This module provides a reusable builder for the standard protocol options
//! layout pattern: ScrolledWindow → Clamp → Box with consistent margins.

use gtk4::prelude::*;
use gtk4::{Box as GtkBox, Orientation, ScrolledWindow};
use libadwaita as adw;

/// Builder for protocol options panel layout.
///
/// Creates the standard layout structure used by all protocol options:
/// - ScrolledWindow (vertical scrolling only)
/// - Clamp (max 600px, tightening at 400px)
/// - Vertical Box with 12px spacing and margins
///
/// # Example
/// ```ignore
/// let (container, content) = ProtocolLayoutBuilder::new()
///     .build();
///
/// // Add preference groups to content
/// content.append(&my_group);
/// ```
#[derive(Debug, Clone)]
pub struct ProtocolLayoutBuilder {
    max_size: i32,
    tightening_threshold: i32,
    spacing: i32,
    margin: i32,
}

impl Default for ProtocolLayoutBuilder {
    fn default() -> Self {
        Self {
            max_size: 600,
            tightening_threshold: 400,
            spacing: 12,
            margin: 12,
        }
    }
}

impl ProtocolLayoutBuilder {
    /// Creates a new builder with default settings.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Builds the layout and returns the container and content box.
    ///
    /// Returns a tuple of:
    /// - The outer container (GtkBox) to be used as the tab content
    /// - The inner content box where preference groups should be added
    #[must_use]
    pub fn build(self) -> (GtkBox, GtkBox) {
        let scrolled = ScrolledWindow::builder()
            .hscrollbar_policy(gtk4::PolicyType::Never)
            .vscrollbar_policy(gtk4::PolicyType::Automatic)
            .vexpand(true)
            .build();

        let clamp = adw::Clamp::builder()
            .maximum_size(self.max_size)
            .tightening_threshold(self.tightening_threshold)
            .build();

        let content = GtkBox::new(Orientation::Vertical, self.spacing);
        content.set_margin_top(self.margin);
        content.set_margin_bottom(self.margin);
        content.set_margin_start(self.margin);
        content.set_margin_end(self.margin);

        clamp.set_child(Some(&content));
        scrolled.set_child(Some(&clamp));

        let container = GtkBox::new(Orientation::Vertical, 0);
        container.append(&scrolled);

        (container, content)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_values() {
        let builder = ProtocolLayoutBuilder::default();
        assert_eq!(builder.max_size, 600);
        assert_eq!(builder.tightening_threshold, 400);
        assert_eq!(builder.spacing, 12);
        assert_eq!(builder.margin, 12);
    }
}
