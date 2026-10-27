//! Wave F W608 / ADR-952 / F-11: live specs show progress without tests
//! pinning historical `W###+` OR ladders. Call sites must not grow those chains.

/// Nail that a construction spec still states progress, without a window-plus OR.
pub fn assert_spec_progress_without_window_plus_or(spec: &str) {
    let progress = spec.contains("已交")
        || spec.contains("delivered")
        || spec.contains("Progress")
        || spec.contains("Next")
        || spec.contains("website")
        || spec.contains("hosted")
        || spec.contains("网站");
    assert!(
        progress,
        "spec must cite progress semantics (已交 / delivered / Progress / Next / website / hosted); do not pin a W###+ OR ladder"
    );
}
