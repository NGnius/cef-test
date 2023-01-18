use super::Feedback;
use super::{TabSelector, ElementSelector};

/// API-specific implementation of interacting with CEF DevTools
pub trait TestAdapter {
    /// Click on element in tab
    fn element_click(&mut self, tab: &TabSelector, element: &ElementSelector) -> Feedback;

    /// Wait for element to appear in tab
    fn element_wait(&mut self, tab: &TabSelector, element: &ElementSelector) -> Feedback;

    /// Focus on element in tab
    fn element_focus(&mut self, tab: &TabSelector, element: &ElementSelector) -> Feedback;

    /// Scroll to element in tab
    fn element_scroll_to(&mut self, tab: &TabSelector, element: &ElementSelector) -> Feedback;

    /// Pause execution in tab for a period
    fn wait(&mut self, tab: &TabSelector, milliseconds: u64) -> Feedback;

    /// Run Javascript in tab
    fn evaluate(&mut self, tab: &TabSelector, script: &str) -> Feedback;
    // TODO
}
