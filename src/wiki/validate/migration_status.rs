use crate::wiki::note::WikiNote;

/// Result of the migration status check.
pub(super) struct MigrationStatus {
    pub total: usize,
    pub without_items: usize,
    /// Paths of notes that have no memory_items (legacy notes).
    pub legacy_paths: Vec<String>,
}

/// Check 11: Report migration status — which notes have memory_items and which don't.
/// This is informational only (warning level, never promoted to error in strict mode).
pub(super) fn check_migration_status(notes: &[WikiNote]) -> MigrationStatus {
    let total = notes.len();
    let legacy_paths: Vec<String> = notes
        .iter()
        .filter(|n| n.memory_items.is_empty())
        .map(|n| n.path.clone())
        .collect();

    let without_items = legacy_paths.len();

    MigrationStatus {
        total,
        without_items,
        legacy_paths,
    }
}
