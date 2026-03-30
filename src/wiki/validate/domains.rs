use std::collections::HashSet;
use std::fs;
use std::path::Path;

use anyhow::{Context, Result};
use walkdir::WalkDir;

use crate::wiki::common::{DOMAIN_PARENT_DIRS, LINK_RE};
use crate::wiki::note::WikiNote;

/// Check 2: Find domains in codebase not documented in .wiki/domains/
pub(super) fn check_undocumented_domains() -> Result<Vec<String>> {
    let project_root = std::env::current_dir().context("Failed to get current directory")?;
    let wiki_domains_dir = Path::new(".wiki/domains");

    // Collect existing wiki domains
    let mut documented: HashSet<String> = HashSet::new();
    if wiki_domains_dir.exists() {
        for entry in fs::read_dir(wiki_domains_dir)? {
            let entry = entry?;
            if entry.path().is_dir() {
                if let Some(name) = entry.file_name().to_str() {
                    documented.insert(name.to_string());
                }
            }
        }
    }

    // Scan codebase for domain-like directories
    let mut code_domains: HashSet<String> = HashSet::new();
    let src_dir = project_root.join("src");

    let search_roots: Vec<std::path::PathBuf> = if src_dir.exists() {
        vec![src_dir]
    } else {
        vec![project_root.clone()]
    };

    for root in &search_roots {
        for entry in WalkDir::new(root)
            .max_depth(4)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            if !entry.file_type().is_dir() {
                continue;
            }

            let path = entry.path();
            if let Some(parent_name) = path
                .parent()
                .and_then(|p| p.file_name())
                .and_then(|n| n.to_str())
            {
                if DOMAIN_PARENT_DIRS.contains(&parent_name.to_lowercase().as_str()) {
                    if let Some(domain_name) = path.file_name().and_then(|n| n.to_str()) {
                        let normalized = domain_name.to_lowercase().replace('_', "-");
                        code_domains.insert(normalized);
                    }
                }
            }
        }
    }

    let mut undocumented: Vec<String> = code_domains.difference(&documented).cloned().collect();
    undocumented.sort();

    Ok(undocumented)
}

/// Check 8: Verify folder name matches the domain field from note content
pub(super) fn check_domain_name_coherence(notes: &[WikiNote]) -> Vec<String> {
    let mut errors = Vec::new();

    for note in notes {
        // Extract the expected domain name from the note's path
        // Path format: .wiki/domains/<domain_name>/_overview.md (or similar)
        let path = Path::new(&note.path);
        let folder_domain = path
            .parent()
            .and_then(|p| p.file_name())
            .and_then(|n| n.to_str())
            .unwrap_or("");

        // Skip non-domain notes (decisions, etc.)
        if folder_domain.is_empty() || !note.path.contains("domains/") || folder_domain == "domains"
        {
            continue;
        }

        // The note's domain field should match the folder name
        if !note.domain.is_empty() && note.domain != folder_domain {
            errors.push(format!(
                "{}: folder name '{}' does not match domain field '{}'",
                note.path, folder_domain, note.domain
            ));
        }
    }

    errors
}

/// Check 9: Verify that domains referenced in Dependencies sections exist in .wiki/domains/
pub(super) fn check_missing_dependencies(notes: &[WikiNote]) -> Vec<(String, String)> {
    // Collect all existing domain names
    let existing_domains: HashSet<String> = notes
        .iter()
        .filter(|n| n.path.contains("domains/"))
        .filter_map(|n| {
            Path::new(&n.path)
                .parent()
                .and_then(|p| p.file_name())
                .and_then(|n| n.to_str())
                .map(|s| s.to_string())
        })
        .collect();

    let mut missing = Vec::new();

    // Check each note's content for a Dependencies section and extract linked domains
    for note in notes {
        if !note.path.contains("domains/") {
            continue;
        }

        let mut in_deps_section = false;
        for line in note.content.lines() {
            if line.starts_with("## Dependencies") || line.starts_with("## Depends on") {
                in_deps_section = true;
                continue;
            }
            if line.starts_with("## ") && in_deps_section {
                break;
            }
            if in_deps_section {
                // Look for links like [domain](../domain/_overview.md) or plain names
                for cap in LINK_RE.captures_iter(line) {
                    let link_text = &cap[1];
                    let link_target = &cap[2];

                    // Extract domain from link target path
                    let dep_domain = Path::new(link_target)
                        .parent()
                        .and_then(|p| p.file_name())
                        .and_then(|n| n.to_str())
                        .unwrap_or(link_text);

                    if !dep_domain.is_empty()
                        && dep_domain != "."
                        && !existing_domains.contains(dep_domain)
                    {
                        missing.push((note.path.clone(), dep_domain.to_string()));
                    }
                }
            }
        }
    }

    missing
}
