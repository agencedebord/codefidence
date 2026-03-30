use crate::wiki::note::{MemoryItem, MemoryItemStatus};
use crate::wiki::prioritize::{confidence_priority, has_related_file, type_priority};

/// Sort and select the top memory items for context injection.
///
/// Priority order:
/// 1. Type: exception > decision > business_rule
/// 2. Confidence: confirmed/verified > seen-in-code > inferred > needs-validation
/// 3. Related file match: items whose related_files match the queried file come first
pub(super) fn prioritize_memory_items<'a>(
    items: &'a [MemoryItem],
    file_path: &str,
    max: usize,
) -> Vec<&'a MemoryItem> {
    let mut active_items: Vec<&MemoryItem> = items
        .iter()
        .filter(|i| matches!(i.status, MemoryItemStatus::Active))
        .collect();

    active_items.sort_by(|a, b| {
        let key_a = (
            type_priority(&a.type_),
            confidence_priority(&a.confidence),
            if has_related_file(a, file_path) {
                0u8
            } else {
                1u8
            },
        );
        let key_b = (
            type_priority(&b.type_),
            confidence_priority(&b.confidence),
            if has_related_file(b, file_path) {
                0u8
            } else {
                1u8
            },
        );
        key_a.cmp(&key_b)
    });

    active_items.into_iter().take(max).collect()
}
