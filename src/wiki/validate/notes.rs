use chrono::Utc;

use crate::wiki::note::{Confidence, WikiNote};

/// Check 5: Count notes by confidence, warn if >40% are low-confidence
pub(super) fn check_confidence_ratio(notes: &[WikiNote]) -> (usize, usize, f64) {
    let total = notes.len();
    if total == 0 {
        return (0, 0, 0.0);
    }

    let low = notes
        .iter()
        .filter(|n| {
            matches!(
                n.confidence,
                Confidence::Inferred | Confidence::NeedsValidation
            )
        })
        .count();

    let pct = low as f64 / total as f64 * 100.0;
    (low, total, pct)
}

/// Check 6: Find notes with last_updated older than the configured staleness threshold
pub(super) fn check_staleness(notes: &[WikiNote], staleness_days: u32) -> Vec<(String, i64)> {
    let today = Utc::now().date_naive();
    let threshold = i64::from(staleness_days);

    notes
        .iter()
        .filter_map(|n| {
            n.last_updated.and_then(|date| {
                let days = (today - date).num_days();
                if days > threshold {
                    Some((n.path.clone(), days))
                } else {
                    None
                }
            })
        })
        .collect()
}
