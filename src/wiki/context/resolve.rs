use std::path::Path;

use anyhow::Result;

use crate::wiki::common::ensure_wiki_exists;
use crate::wiki::file_index;
use crate::wiki::note::{Confidence, MemoryItem, WikiNote};

use super::prioritize::prioritize_memory_items;
use super::render::compact_summary;
use super::{ContextJsonItem, ContextJsonOutput, MAX_MEMORY_ITEMS};

/// Core logic: resolve wiki context for a given file path.
pub fn resolve_context(
    file_path: &str,
    wiki_dir: &Path,
    project_root: &Path,
) -> Result<Option<String>> {
    ensure_wiki_exists(wiki_dir)?;

    let index = file_index::load_or_rebuild(wiki_dir)?;

    let domain = match file_index::resolve_domain(&index, file_path, project_root) {
        Some(d) => d,
        None => return Ok(None),
    };

    // Read the domain's _overview.md
    let overview_path = wiki_dir.join("domains").join(&domain).join("_overview.md");
    if !overview_path.exists() {
        return Ok(None);
    }

    let note = WikiNote::parse(&overview_path)?;
    Ok(Some(compact_summary(&note, &domain, file_path)))
}

/// Resolve context as structured JSON for a file.
pub(super) fn resolve_context_json(
    file_path: &str,
    wiki_dir: &Path,
    project_root: &Path,
) -> Result<ContextJsonOutput> {
    ensure_wiki_exists(wiki_dir)?;

    let index = file_index::load_or_rebuild(wiki_dir)?;

    let domain = file_index::resolve_domain(&index, file_path, project_root);

    if domain.is_none() {
        return Ok(ContextJsonOutput {
            schema_version: "1".to_string(),
            domain: None,
            confidence: None,
            last_updated: None,
            memory_items: Vec::new(),
            warnings: vec!["No domain found for this file".to_string()],
            fallback_mode: false,
        });
    }

    let domain = domain.unwrap();
    let overview_path = wiki_dir.join("domains").join(&domain).join("_overview.md");

    if !overview_path.exists() {
        return Ok(ContextJsonOutput {
            schema_version: "1".to_string(),
            domain: Some(domain),
            confidence: None,
            last_updated: None,
            memory_items: Vec::new(),
            warnings: vec!["Domain overview not found".to_string()],
            fallback_mode: false,
        });
    }

    let note = WikiNote::parse(&overview_path)?;
    let fallback_mode = note.memory_items.is_empty();

    let prioritized = prioritize_memory_items(&note.memory_items, file_path, MAX_MEMORY_ITEMS);
    let items: Vec<ContextJsonItem> = prioritized
        .into_iter()
        .map(|item| ContextJsonItem {
            id: item.id.clone(),
            type_: item.type_.to_string(),
            text: item.text.clone(),
            confidence: item.confidence.to_string(),
        })
        .collect();

    let mut warnings = Vec::new();
    let low_confidence_items: Vec<&MemoryItem> = note
        .memory_items
        .iter()
        .filter(|i| {
            matches!(
                i.confidence,
                Confidence::Inferred | Confidence::NeedsValidation
            )
        })
        .collect();
    if !low_confidence_items.is_empty() {
        warnings.push(format!(
            "{} item(s) have low confidence — verify before relying on them",
            low_confidence_items.len()
        ));
    }

    Ok(ContextJsonOutput {
        schema_version: "1".to_string(),
        domain: Some(domain),
        confidence: Some(note.confidence.to_string()),
        last_updated: note.last_updated.map(|d| d.to_string()),
        memory_items: items,
        warnings,
        fallback_mode,
    })
}
