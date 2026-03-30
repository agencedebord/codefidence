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

    if let Some(ctx) = resolve_context(file_path, &wiki_dir, project_root)? {
        let output = HookOutput {
            additional_context: ctx,
        };
        println!("{}", serde_json::to_string(&output)?);
    }

    Ok(())
}
