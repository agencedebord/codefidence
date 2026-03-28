mod dependencies;
mod details;
mod generate;
mod imports;
pub mod structure;

use std::collections::HashMap;
use std::path::PathBuf;

use anyhow::{Context, Result};

use crate::ui;

pub use generate::{
    generate_domain_overview, generate_graph, generate_index, generate_needs_review,
};

// ─── Public types ───

#[derive(Debug, Clone)]
pub struct DomainInfo {
    pub name: String,
    pub files: Vec<String>,
    pub dependencies: Vec<String>,
    pub models: Vec<String>,
    pub routes: Vec<String>,
    pub comments: Vec<String>,
    pub test_files: Vec<String>,
}

#[derive(Debug)]
pub struct ScanResult {
    pub domains: Vec<DomainInfo>,
    pub total_files_scanned: usize,
    pub languages_detected: Vec<String>,
}

// ─── Main entry point ───

pub fn run() -> Result<ScanResult> {
    let project_root = std::env::current_dir().context("Failed to get current directory")?;

    ui::action("Scanning codebase");
    eprintln!();

    // Pass 1: Structure discovery
    ui::step("Pass 1 — discovering project structure...");
    let (all_files, domains_map) = structure::discover_structure(&project_root)?;
    let total_files = all_files.len();

    let languages = structure::detect_languages(&all_files);
    ui::scan_progress(
        &format!(
            "{} files found, {} languages detected",
            total_files,
            languages.len()
        ),
        0.33,
    );

    if domains_map.is_empty() {
        ui::info("No domain candidates found. The wiki will start empty.");
        return Ok(ScanResult {
            domains: Vec::new(),
            total_files_scanned: total_files,
            languages_detected: languages,
        });
    }

    ui::step(&format!(
        "Found {} domain candidate(s): {}",
        domains_map.len(),
        domains_map.keys().cloned().collect::<Vec<_>>().join(", ")
    ));

    for (name, files) in &domains_map {
        ui::verbose(&format!("domain {:?} — {} file(s)", name, files.len()));
    }

    // Pass 2: Relationship analysis
    ui::step("Pass 2 — analyzing cross-domain dependencies...");
    let source_files: Vec<&PathBuf> = all_files
        .iter()
        .filter(|p| structure::is_source_file(p))
        .collect();

    let file_imports = imports::extract_all_imports(&source_files, &project_root);
    let dependency_graph =
        dependencies::build_dependency_graph(&domains_map, &file_imports, &project_root);
    ui::scan_progress(
        &format!("{} source files analyzed for imports", source_files.len()),
        0.66,
    );

    // Pass 3: Detail extraction
    ui::step("Pass 3 — extracting models, routes, and TODOs...");
    let mut domains: Vec<DomainInfo> = Vec::new();

    let domain_names: Vec<String> = domains_map.keys().cloned().collect();
    for (i, name) in domain_names.iter().enumerate() {
        let files = &domains_map[name];
        let extracted = details::extract_details(files, &project_root);

        let deps = dependency_graph.get(name).cloned().unwrap_or_default();

        let test_files: Vec<String> = files
            .iter()
            .filter(|f| structure::is_test_file(f))
            .map(|f| structure::relativize(f, &project_root))
            .collect();

        let relative_files: Vec<String> = files
            .iter()
            .map(|f| structure::relativize(f, &project_root))
            .collect();

        domains.push(DomainInfo {
            name: name.clone(),
            files: relative_files,
            dependencies: deps,
            models: extracted.models,
            routes: extracted.routes,
            comments: extracted.comments,
            test_files,
        });

        let progress = 0.66 + 0.34 * ((i + 1) as f64 / domain_names.len() as f64);
        ui::scan_progress(&format!("Extracted details for {}", name), progress);
    }

    // Sort domains alphabetically
    domains.sort_by(|a, b| a.name.cmp(&b.name));

    eprintln!();
    ui::success(&format!(
        "Scan complete: {} domains, {} files, {} languages",
        domains.len(),
        total_files,
        languages.len()
    ));

    Ok(ScanResult {
        domains,
        total_files_scanned: total_files,
        languages_detected: languages,
    })
}

// Re-export the DomainFileMap type for internal use
pub(crate) type DomainFileMap = HashMap<String, Vec<PathBuf>>;
