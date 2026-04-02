use std::path::Path;
use std::process::Command;

use anyhow::{Context, Result, bail};
use serde::Deserialize;

use super::scan::DomainInfo;
use crate::ui;

// ─── Constants ───

const CLAUDE_MODEL: &str = "sonnet";
const MAX_FILE_SNIPPETS: usize = 10;
const SNIPPET_LINE_LIMIT: usize = 200;
const MAX_CONTEXT_CHARS: usize = 40_000;

// ─── Public types ───

#[derive(Debug, Clone, Deserialize)]
pub struct LlmAnalysis {
    pub description: String,
    #[serde(default)]
    pub behaviors: Vec<Behavior>,
    #[serde(default)]
    pub interactions: Vec<Interaction>,
    #[serde(default)]
    pub gotchas: Vec<String>,
    #[serde(default)]
    pub memory_candidates: Vec<LlmCandidate>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Behavior {
    pub summary: String,
    pub detail: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Interaction {
    pub target_domain: String,
    pub description: String,
}

#[derive(Debug, Clone, Deserialize)]
#[allow(dead_code)] // Fields read via deserialization and used by candidates system
pub struct LlmCandidate {
    #[serde(rename = "type")]
    pub type_: String,
    pub text: String,
}

struct FileSnippet {
    path: String,
    content: String,
}

// ─── JSON Schema for structured output ───

fn json_schema() -> &'static str {
    r#"{"type":"object","properties":{"description":{"type":"string"},"behaviors":{"type":"array","items":{"type":"object","properties":{"summary":{"type":"string"},"detail":{"type":"string"}},"required":["summary","detail"]}},"interactions":{"type":"array","items":{"type":"object","properties":{"target_domain":{"type":"string"},"description":{"type":"string"}},"required":["target_domain","description"]}},"gotchas":{"type":"array","items":{"type":"string"}},"memory_candidates":{"type":"array","items":{"type":"object","properties":{"type":{"type":"string","enum":["exception","decision","business_rule"]},"text":{"type":"string"}},"required":["type","text"]}}},"required":["description"]}"#
}

// ─── Public entry point ───

/// Analyze all domains using Claude Code CLI to produce real documentation.
/// Returns a vec of (domain_name, analysis) pairs.
/// Errors on individual domains are logged as warnings, never abort.
pub fn run(
    domains: &[DomainInfo],
    all_domains: &[DomainInfo],
    _wiki_dir: &Path,
) -> Result<Vec<(String, LlmAnalysis)>> {
    ensure_claude_available()?;

    let total = domains.len();
    let mut results: Vec<(String, LlmAnalysis)> = Vec::new();

    for (i, domain) in domains.iter().enumerate() {
        let progress = (i + 1) as f64 / total as f64;
        ui::llm_progress(&format!("Analyzing {}...", domain.name), progress);

        match analyze_domain(domain, all_domains) {
            Ok(analysis) => {
                results.push((domain.name.clone(), analysis));
            }
            Err(e) => {
                ui::warn(&format!(
                    "Failed to analyze {}: {}. Skipping.",
                    domain.name, e
                ));
            }
        }
    }

    ui::success(&format!(
        "LLM analysis complete for {}/{} domain(s).",
        results.len(),
        total
    ));

    Ok(results)
}

// ─── Claude CLI detection ───

fn ensure_claude_available() -> Result<()> {
    // Check that claude CLI is installed
    match Command::new("claude").arg("--version").output() {
        Ok(output) if output.status.success() => {}
        _ => bail!(
            "Claude Code is required for AI analysis.\n\
             Install it with: npm install -g @anthropic-ai/claude-code\n\
             Or use --scan-only for structural-only bootstrap."
        ),
    }

    // Quick auth check: run a trivial prompt to verify authentication
    let has_api_key = std::env::var("ANTHROPIC_API_KEY")
        .map(|v| !v.trim().is_empty())
        .unwrap_or(false);
    let mut test_args = vec![
        "-p",
        "ok",
        "--output-format",
        "json",
        "--no-session-persistence",
    ];
    if has_api_key {
        test_args.push("--bare");
    }
    let test = Command::new("claude").args(&test_args).output();

    if let Ok(output) = test {
        let stdout = String::from_utf8_lossy(&output.stdout);
        if let Ok(envelope) = serde_json::from_str::<serde_json::Value>(&stdout) {
            if envelope["is_error"].as_bool() == Some(true) {
                if let Some(result) = envelope["result"].as_str() {
                    if result.to_lowercase().contains("not logged in")
                        || result.to_lowercase().contains("login")
                    {
                        bail!(
                            "Claude Code is not authenticated.\n\
                             Run `claude` in a terminal and complete login,\n\
                             or set the ANTHROPIC_API_KEY environment variable.\n\
                             Then retry: codefidence init --scan\n\
                             Or use --scan-only for structural-only bootstrap."
                        );
                    }
                }
            }
        }
    }

    Ok(())
}

// ─── Domain analysis ───

fn analyze_domain(domain: &DomainInfo, all_domains: &[DomainInfo]) -> Result<LlmAnalysis> {
    let snippets = collect_file_snippets(domain);
    if snippets.is_empty() {
        bail!("No source files available to analyze");
    }
    let prompt = build_prompt(domain, all_domains, &snippets);
    let mut response = call_claude(&prompt)?;
    validate_response(&mut response);
    Ok(response)
}

/// Call Claude Code CLI in print mode with structured JSON output.
///
/// Uses `--bare` mode (fast, no hooks/CLAUDE.md) when ANTHROPIC_API_KEY is set.
/// Falls back to normal mode (keychain/OAuth auth) otherwise.
fn call_claude(prompt: &str) -> Result<LlmAnalysis> {
    let has_api_key = std::env::var("ANTHROPIC_API_KEY")
        .map(|v| !v.trim().is_empty())
        .unwrap_or(false);
    let schema = json_schema();

    let mut cmd = Command::new("claude");
    cmd.args([
        "-p",
        prompt,
        "--output-format",
        "json",
        "--model",
        CLAUDE_MODEL,
        "--json-schema",
        schema,
        "--no-session-persistence",
    ]);

    if has_api_key {
        // Fast path: no hooks, no CLAUDE.md, no keychain reads
        cmd.arg("--bare");
    }

    let output = cmd.output().context("Failed to execute claude CLI")?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        let stdout = String::from_utf8_lossy(&output.stdout);

        // Claude CLI returns errors as JSON in stdout with is_error: true
        if let Ok(envelope) = serde_json::from_str::<serde_json::Value>(&stdout) {
            if let Some(result) = envelope["result"].as_str() {
                if !result.is_empty() {
                    bail!("Claude CLI error: {}", result);
                }
            }
            // If we got JSON but no result text, show the full response for debugging
            bail!(
                "Claude CLI failed with JSON response: {}",
                stdout.chars().take(500).collect::<String>()
            );
        }

        // No JSON in stdout, show stderr
        let error_detail = if stderr.trim().is_empty() {
            format!("stdout: {}", stdout.chars().take(500).collect::<String>())
        } else {
            stderr.trim().to_string()
        };
        bail!(
            "Claude CLI failed (exit code {}): {}",
            output.status,
            error_detail
        );
    }

    let stdout = String::from_utf8_lossy(&output.stdout);

    // claude --output-format json wraps the result in a JSON envelope
    let envelope: serde_json::Value =
        serde_json::from_str(&stdout).context("Failed to parse Claude CLI JSON output")?;

    // With --json-schema, Claude returns structured output in `structured_output`.
    // Try to parse it directly as LlmAnalysis.
    if let Some(structured) = envelope.get("structured_output") {
        if let Some(obj) = structured.as_object() {
            if !obj.is_empty() {
                // If the object has a "description" key, it matches our schema directly
                let json_to_parse = if obj.contains_key("description") {
                    structured.to_string()
                } else if obj.len() == 1 {
                    // Single-key wrapper: the value is either a JSON string or object
                    let value = obj.values().next().unwrap();
                    if let Some(s) = value.as_str() {
                        s.to_string()
                    } else {
                        value.to_string()
                    }
                } else {
                    structured.to_string()
                };

                match serde_json::from_str::<LlmAnalysis>(&json_to_parse) {
                    Ok(parsed) => return Ok(parsed),
                    Err(e) => {
                        crate::ui::verbose(&format!(
                            "structured_output parse failed ({}), falling back to result text",
                            e
                        ));
                    }
                }
            }
        }
    }

    // Fallback: parse from "result" text field
    let result_text = envelope["result"].as_str().ok_or_else(|| {
        anyhow::anyhow!(
            "Unexpected Claude CLI output structure: no 'result' field.\nFull output: {}",
            stdout.chars().take(500).collect::<String>()
        )
    })?;

    // Strip markdown fencing if Claude adds it despite --json-schema
    let clean = strip_json_fencing(result_text);

    let parsed: LlmAnalysis = serde_json::from_str(clean).with_context(|| {
        format!(
            "Failed to parse Claude response as LlmAnalysis: {}",
            clean.chars().take(500).collect::<String>()
        )
    })?;

    Ok(parsed)
}

fn collect_file_snippets(domain: &DomainInfo) -> Vec<FileSnippet> {
    let mut candidates: Vec<&String> = domain
        .files
        .iter()
        .filter(|f| !domain.test_files.contains(f))
        .filter(|f| !is_noise_path(f))
        .collect();

    // Sort: files with model/route-related names first, then by path length (shorter = more central)
    candidates.sort_by(|a, b| {
        let a_score = file_priority_score(a, domain);
        let b_score = file_priority_score(b, domain);
        b_score.cmp(&a_score).then(a.len().cmp(&b.len()))
    });

    let mut snippets: Vec<FileSnippet> = Vec::new();
    let mut total_chars: usize = 0;

    for path in candidates.into_iter().take(MAX_FILE_SNIPPETS) {
        let content = match std::fs::read_to_string(path) {
            Ok(c) => c,
            Err(_) => continue,
        };

        let lines: Vec<&str> = content.lines().take(SNIPPET_LINE_LIMIT).collect();
        if lines.is_empty() {
            continue;
        }

        let snippet_content = lines.join("\n");
        let snippet_len = snippet_content.len(); // byte count, sufficient for ASCII-heavy code

        // Budget check: stop if adding this snippet would exceed the budget
        if total_chars + snippet_len > MAX_CONTEXT_CHARS && !snippets.is_empty() {
            let remaining = MAX_CONTEXT_CHARS.saturating_sub(total_chars);
            if remaining > 500 {
                // Truncate at a char boundary to avoid panic on multi-byte UTF-8
                let mut end = remaining.min(snippet_content.len());
                while end > 0 && !snippet_content.is_char_boundary(end) {
                    end -= 1;
                }
                snippets.push(FileSnippet {
                    path: path.clone(),
                    content: snippet_content[..end].to_string(),
                });
            }
            break;
        }

        total_chars += snippet_len;
        snippets.push(FileSnippet {
            path: path.clone(),
            content: snippet_content,
        });

        if total_chars >= MAX_CONTEXT_CHARS {
            break;
        }
    }

    snippets
}

/// Score a file for snippet priority (higher = more important).
fn file_priority_score(path: &str, domain: &DomainInfo) -> u32 {
    let lower = path.to_lowercase();
    let mut score = 0;

    for model in &domain.models {
        if lower.contains(&model.to_lowercase()) {
            score += 3;
            break;
        }
    }

    if lower.contains("route")
        || lower.contains("controller")
        || lower.contains("handler")
        || lower.contains("api")
    {
        score += 2;
    }

    if lower.ends_with("mod.rs")
        || lower.ends_with("index.ts")
        || lower.ends_with("index.js")
        || lower.ends_with("__init__.py")
    {
        score += 1;
    }

    score
}

/// Check if a path is "noise" that shouldn't be sent as snippet to the LLM.
fn is_noise_path(path: &str) -> bool {
    let lower = path.to_lowercase();
    lower.contains("/migrations/")
        || lower.contains("/templates/")
        || lower.contains("/static/")
        || lower.contains("/media/")
        || lower.contains("/locale/")
        || lower.contains("/fixtures/")
}

fn build_prompt(
    domain: &DomainInfo,
    all_domains: &[DomainInfo],
    snippets: &[FileSnippet],
) -> String {
    let deps_str = if domain.dependencies.is_empty() {
        "none".to_string()
    } else {
        domain.dependencies.join(", ")
    };

    let referenced_by: Vec<&str> = all_domains
        .iter()
        .filter(|d| d.name != domain.name && d.dependencies.contains(&domain.name))
        .map(|d| d.name.as_str())
        .collect();
    let referenced_by_str = if referenced_by.is_empty() {
        "none".to_string()
    } else {
        referenced_by.join(", ")
    };

    let models_str = if domain.models.is_empty() {
        "none detected".to_string()
    } else {
        domain.models.join(", ")
    };

    let routes_str = if domain.routes.is_empty() {
        "none detected".to_string()
    } else {
        domain.routes.join(", ")
    };

    let comments_str = if domain.comments.is_empty() {
        "none".to_string()
    } else {
        domain.comments.join("; ")
    };

    let snippets_str = snippets
        .iter()
        .map(|s| format!("### {}\n```\n{}\n```", s.path, s.content))
        .collect::<Vec<_>>()
        .join("\n\n");

    format!(
        r#"You are documenting a software domain called "{domain}" for a project knowledge base.
Your audience is an LLM that will work on this codebase. Write documentation that explains things the code alone doesn't make obvious.

## Structural context

- Dependencies (this domain imports from): {deps}
- Referenced by (other domains that import this one): {referenced_by}
- Models/types detected: {models}
- API routes detected: {routes}
- Code comments (TODO/FIXME/HACK): {comments}

## Source code

{snippets}

## Instructions

Analyze the source code above and produce documentation. Focus on:
1. **What this domain does functionally** — its purpose, the problems it solves
2. **Business rules and logic** embedded in the code — validation rules, constraints, state machines, special cases
3. **Why things are done this way** — when the code reveals intentional design choices or trade-offs
4. **How this domain interacts with its dependencies** — what it needs from them, what it provides to others
5. **Edge cases, gotchas, and non-obvious behaviors** — things that would surprise a developer working on this code

DO NOT list models, routes, or file counts — the reader can see those in code.
DO NOT describe what the code "contains" — describe what it DOES and WHY.
Be factual. Only document what the code evidence supports. If something is unclear, say so."#,
        domain = domain.name,
        deps = deps_str,
        referenced_by = referenced_by_str,
        models = models_str,
        routes = routes_str,
        comments = comments_str,
        snippets = snippets_str,
    )
}

/// Strip markdown JSON fencing (```json ... ```) if the LLM adds it.
fn strip_json_fencing(text: &str) -> &str {
    let trimmed = text.trim();

    // Handle ```json, ```JSON, ```Json, etc.
    let lower_prefix = trimmed.get(..7).map(|s| s.to_lowercase());
    if lower_prefix.as_deref() == Some("```json") {
        if let Some(inner) = trimmed[7..].strip_suffix("```") {
            return inner.trim();
        }
    }

    if let Some(rest) = trimmed.strip_prefix("```") {
        if let Some(inner) = rest.strip_suffix("```") {
            return inner.trim();
        }
    }

    trimmed
}

// ─── Response validation ───

fn validate_response(resp: &mut LlmAnalysis) {
    resp.behaviors.truncate(5);
    resp.interactions.truncate(10);
    resp.gotchas.truncate(5);

    resp.memory_candidates
        .retain(|c| matches!(c.type_.as_str(), "exception" | "decision" | "business_rule"));
    resp.memory_candidates.truncate(3);

    if resp.description.len() > 600 {
        // Find a char boundary to avoid panic on multi-byte UTF-8
        let mut end = 597;
        while !resp.description.is_char_boundary(end) && end > 0 {
            end -= 1;
        }
        resp.description.truncate(end);
        resp.description.push_str("...");
    }
}

// ─── Tests ───

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn strip_json_fencing_plain() {
        let input = r#"{"description": "test"}"#;
        assert_eq!(strip_json_fencing(input), input);
    }

    #[test]
    fn strip_json_fencing_with_json_tag() {
        let input = "```json\n{\"description\": \"test\"}\n```";
        assert_eq!(strip_json_fencing(input), "{\"description\": \"test\"}");
    }

    #[test]
    fn strip_json_fencing_with_bare_backticks() {
        let input = "```\n{\"description\": \"test\"}\n```";
        assert_eq!(strip_json_fencing(input), "{\"description\": \"test\"}");
    }

    #[test]
    fn validate_response_truncates() {
        let mut resp = LlmAnalysis {
            description: "x".repeat(700),
            behaviors: vec![
                Behavior {
                    summary: "a".into(),
                    detail: "d".into(),
                },
                Behavior {
                    summary: "b".into(),
                    detail: "d".into(),
                },
                Behavior {
                    summary: "c".into(),
                    detail: "d".into(),
                },
                Behavior {
                    summary: "d".into(),
                    detail: "d".into(),
                },
                Behavior {
                    summary: "e".into(),
                    detail: "d".into(),
                },
                Behavior {
                    summary: "f".into(),
                    detail: "d".into(),
                },
            ],
            interactions: vec![],
            gotchas: vec![],
            memory_candidates: vec![
                LlmCandidate {
                    type_: "decision".to_string(),
                    text: "ok".to_string(),
                },
                LlmCandidate {
                    type_: "invalid_type".to_string(),
                    text: "bad".to_string(),
                },
            ],
        };

        validate_response(&mut resp);

        assert_eq!(resp.description.len(), 600);
        assert!(resp.description.ends_with("..."));
        assert_eq!(resp.behaviors.len(), 5);
        assert_eq!(resp.memory_candidates.len(), 1);
        assert_eq!(resp.memory_candidates[0].type_, "decision");
    }

    #[test]
    fn file_priority_score_model_match() {
        let domain = DomainInfo {
            name: "billing".to_string(),
            files: vec![],
            dependencies: vec![],
            models: vec!["Invoice".to_string()],
            routes: vec![],
            comments: vec![],
            test_files: vec![],
        };

        assert!(file_priority_score("src/billing/invoice.rs", &domain) >= 3);
        assert_eq!(file_priority_score("src/billing/utils.rs", &domain), 0);
    }

    #[test]
    fn file_priority_score_route_patterns() {
        let domain = DomainInfo {
            name: "billing".to_string(),
            files: vec![],
            dependencies: vec![],
            models: vec![],
            routes: vec![],
            comments: vec![],
            test_files: vec![],
        };

        assert!(file_priority_score("src/billing/controller.ts", &domain) >= 2);
        assert!(file_priority_score("src/billing/handler.rs", &domain) >= 2);
        assert!(file_priority_score("src/api/billing.ts", &domain) >= 2);
    }

    #[test]
    fn parse_llm_analysis_response() {
        let json = r#"{
            "description": "Handles billing operations including invoice creation and payment processing.",
            "behaviors": [
                {"summary": "Invoice validation", "detail": "Validates invoice amounts are positive and within credit limits before persisting."},
                {"summary": "Payment retry", "detail": "Failed payments are retried up to 3 times with exponential backoff."}
            ],
            "interactions": [
                {"target_domain": "users", "description": "Fetches customer billing profiles to determine credit limits."}
            ],
            "gotchas": ["Refunds older than 30 days silently fail without error"],
            "memory_candidates": [{"type": "business_rule", "text": "Invoices expire after 30 days"}]
        }"#;

        let resp: LlmAnalysis = serde_json::from_str(json).unwrap();
        assert_eq!(resp.behaviors.len(), 2);
        assert_eq!(resp.interactions.len(), 1);
        assert_eq!(resp.gotchas.len(), 1);
        assert_eq!(resp.memory_candidates.len(), 1);
        assert_eq!(resp.memory_candidates[0].type_, "business_rule");
    }

    #[test]
    fn parse_llm_analysis_minimal() {
        let json = r#"{"description": "A simple domain"}"#;
        let resp: LlmAnalysis = serde_json::from_str(json).unwrap();
        assert_eq!(resp.description, "A simple domain");
        assert!(resp.behaviors.is_empty());
        assert!(resp.interactions.is_empty());
        assert!(resp.gotchas.is_empty());
        assert!(resp.memory_candidates.is_empty());
    }

    #[test]
    fn build_prompt_includes_referenced_by() {
        let domain = DomainInfo {
            name: "users".to_string(),
            files: vec!["src/users/mod.rs".to_string()],
            dependencies: vec![],
            models: vec![],
            routes: vec![],
            comments: vec![],
            test_files: vec![],
        };
        let all_domains = vec![
            domain.clone(),
            DomainInfo {
                name: "billing".to_string(),
                files: vec![],
                dependencies: vec!["users".to_string()],
                models: vec![],
                routes: vec![],
                comments: vec![],
                test_files: vec![],
            },
        ];

        let snippets = vec![FileSnippet {
            path: "src/users/mod.rs".to_string(),
            content: "pub fn get_user() {}".to_string(),
        }];

        let prompt = build_prompt(&domain, &all_domains, &snippets);
        assert!(prompt.contains("Referenced by"));
        assert!(prompt.contains("billing"));
    }

    #[test]
    fn json_schema_is_valid_json() {
        let schema: serde_json::Value = serde_json::from_str(json_schema()).unwrap();
        assert_eq!(schema["type"], "object");
        assert!(schema["properties"]["description"].is_object());
        assert!(schema["properties"]["behaviors"].is_object());
    }
}
