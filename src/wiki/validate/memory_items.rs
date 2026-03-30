use std::collections::{HashMap, HashSet};

use chrono::Utc;

use crate::wiki::note::{Confidence, MemoryItemStatus, WikiNote};

/// Check 10: Validate memory_items for structural integrity across all notes
pub(super) fn check_memory_items(notes: &[WikiNote]) -> (Vec<String>, Vec<String>) {
    let mut errors: Vec<String> = Vec::new();
    let mut warnings: Vec<String> = Vec::new();
    let mut all_ids: HashMap<String, String> = HashMap::new(); // id -> first note path
    let today = Utc::now().date_naive();

    for note in notes {
        let mut note_ids: HashSet<String> = HashSet::new();

        for item in &note.memory_items {
            // ── Duplicate id within same note ──
            if !note_ids.insert(item.id.clone()) {
                errors.push(format!("{}: duplicate item id '{}'", note.path, item.id));
            }

            // ── Duplicate id across notes ──
            if let Some(existing_path) = all_ids.get(&item.id) {
                if existing_path != &note.path {
                    errors.push(format!(
                        "{}: item id '{}' already used in {}",
                        note.path, item.id, existing_path
                    ));
                }
            } else {
                all_ids.insert(item.id.clone(), note.path.clone());
            }

            // ── Source validation ──
            for (i, source) in item.sources.iter().enumerate() {
                if source.kind.is_empty() {
                    errors.push(format!(
                        "{}: item '{}' source #{} has empty kind",
                        note.path,
                        item.id,
                        i + 1
                    ));
                }
                if source.ref_.is_empty() {
                    errors.push(format!(
                        "{}: item '{}' source #{} has empty ref",
                        note.path,
                        item.id,
                        i + 1
                    ));
                }
            }

            // ── Confidence inconsistency ──
            if item.is_high_confidence()
                && matches!(
                    note.confidence,
                    Confidence::Inferred | Confidence::NeedsValidation
                )
            {
                warnings.push(format!(
                    "{}: item '{}' is {} but note is {}",
                    note.path, item.id, item.confidence, note.confidence
                ));
            }

            // ── Future date ──
            if let Some(date) = item.last_reviewed_date() {
                if date > today {
                    warnings.push(format!(
                        "{}: item '{}' has last_reviewed in the future ({})",
                        note.path, item.id, date
                    ));
                }
            }

            // ── No sources ──
            if item.sources.is_empty() {
                warnings.push(format!("{}: item '{}' has no sources", note.path, item.id));
            }

            // ── Deprecated item is the only active one ──
            if matches!(item.status, MemoryItemStatus::Deprecated) {
                let active_count = note
                    .memory_items
                    .iter()
                    .filter(|i| matches!(i.status, MemoryItemStatus::Active))
                    .count();
                if active_count == 0 {
                    warnings.push(format!("{}: all memory items are deprecated", note.path));
                }
            }
        }
    }

    (errors, warnings)
}
