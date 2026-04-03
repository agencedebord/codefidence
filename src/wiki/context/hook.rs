use std::io::Read as _;

use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::wiki::common::find_wiki_root;

use super::resolve::resolve_context;

#[derive(Deserialize)]
struct HookInput {
    tool_input: serde_json::Value,
    #[allow(dead_code)]
    cwd: Option<String>,
}

#[derive(Serialize)]
struct HookOutput {
    #[serde(rename = "additionalContext")]
    additional_context: String,
}

/// Hook entry point: read JSON from stdin, write JSON to stdout.
pub fn run_from_stdin() -> Result<()> {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input)?;

    let hook_input: HookInput = serde_json::from_str(&input).unwrap_or(HookInput {
        tool_input: serde_json::Value::Null,
        cwd: None,
    });

    // Extract file_path from tool_input
    let file_path = hook_input
        .tool_input
        .get("file_path")
        .and_then(|v| v.as_str())
        .unwrap_or("");

    if file_path.is_empty() {
        // No file path in the hook input — nothing to do
        return Ok(());
    }

    // Try to find the wiki, but don't fail if it doesn't exist
    let wiki_dir = match find_wiki_root() {
        Ok(dir) => dir,
        Err(_) => return Ok(()), // No wiki — silent exit
    };

    let project_root = match wiki_dir.parent() {
        Some(root) => root,
        None => return Ok(()),
    };

    let mut context_parts: Vec<String> = Vec::new();

    // Check for drift-pending from git hooks
    if let Some(pending) = crate::wiki::git_hook::read_drift_pending(&wiki_dir) {
        let mut warning = format!(
            "\u{26a0} Wiki drift detected from recent git {}.",
            pending.event.replace("post-", "")
        );
        if !pending.domains.is_empty() {
            warning.push_str(&format!(
                " Domains potentially affected: {}.",
                pending.domains.join(", ")
            ));
        }
        if !pending.untracked_files.is_empty() {
            warning.push_str(&format!(
                " New files not covered by wiki: {}. Consider adding them to existing domains or creating new ones.",
                pending.untracked_files.join(", ")
            ));
        }
        warning.push_str(" Consider running `codefidence check-diff` before making changes.");
        context_parts.push(warning);

        // Consume the marker
        crate::wiki::git_hook::consume_drift_pending(&wiki_dir);
    }

    // Normal context resolution
    if let Some(ctx) = resolve_context(file_path, &wiki_dir, project_root)? {
        context_parts.push(ctx);
    }

    if !context_parts.is_empty() {
        let output = HookOutput {
            additional_context: context_parts.join("\n\n"),
        };
        println!("{}", serde_json::to_string(&output)?);
    }

    Ok(())
}
