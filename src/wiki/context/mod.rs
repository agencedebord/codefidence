mod hook;
mod prioritize;
mod render;
mod resolve;

#[cfg(test)]
mod tests;

use anyhow::Result;
use serde::Serialize;

use crate::wiki::common::find_wiki_root;

use resolve::resolve_context_json;

// ── Constants ──────────────────────────────────────────────────────

/// Maximum length for the compact context injected into Claude's context window.
const MAX_CONTEXT_LEN: usize = 2000;

/// Maximum number of memory items to include in context output.
const MAX_MEMORY_ITEMS: usize = 3;

// ── Public types ───────────────────────────────────────────────────

/// JSON output for `context --json`.
#[derive(Debug, Serialize)]
pub struct ContextJsonOutput {
    pub schema_version: String,
    pub domain: Option<String>,
    pub confidence: Option<String>,
    pub last_updated: Option<String>,
    pub memory_items: Vec<ContextJsonItem>,
    pub warnings: Vec<String>,
    pub fallback_mode: bool,
}

/// JSON representation of a memory item in context output.
#[derive(Debug, Serialize)]
pub struct ContextJsonItem {
    pub id: String,
    #[serde(rename = "type")]
    pub type_: String,
    pub text: String,
    pub confidence: String,
}

// ── Public API ─────────────────────────────────────────────────────

pub use hook::run_from_stdin;
pub use resolve::resolve_context;

/// CLI entry point: print context for a file to stdout.
pub fn run(file: &str, json: bool) -> Result<()> {
    let wiki_dir = find_wiki_root()?;

    let project_root = wiki_dir
        .parent()
        .ok_or_else(|| anyhow::anyhow!("Wiki directory has no parent"))?;

    if json {
        let output = resolve_context_json(file, &wiki_dir, project_root)?;
        println!(
            "{}",
            serde_json::to_string_pretty(&output)
                .map_err(|e| anyhow::anyhow!("JSON serialization: {e}"))?
        );
        return Ok(());
    }

    match resolve_context(file, &wiki_dir, project_root)? {
        Some(ctx) => {
            println!("{}", ctx);
            Ok(())
        }
        None => {
            eprintln!("[project-wiki] No wiki context found for: {}", file);
            Ok(())
        }
    }
}
