use std::fs;
use std::path::PathBuf;
use std::time::Duration;

use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

const CRATE_NAME: &str = "codefidence";
const CURRENT_VERSION: &str = env!("CARGO_PKG_VERSION");
const CACHE_TTL_HOURS: i64 = 24;
const HTTP_TIMEOUT_SECS: u64 = 3;

// ─── Cache types ───

#[derive(Debug, Serialize, Deserialize)]
struct UpdateCache {
    latest_version: String,
    checked_at: String,
}

// ─── crates.io API response ───

#[derive(Deserialize)]
struct CratesIoResponse {
    #[serde(rename = "crate")]
    krate: CrateInfo,
}

#[derive(Deserialize)]
struct CrateInfo {
    max_version: String,
}

// ─── Public API ───

/// Non-failing check: returns a warning message if outdated, None otherwise.
/// All errors are silently swallowed — this must never break the CLI.
pub fn check_background() -> Option<String> {
    if std::env::var("CODEFIDENCE_NO_UPDATE_CHECK").ok().as_deref() == Some("1") {
        return None;
    }

    check_inner(false).ok().flatten()
}

/// Force check (bypass cache). Used by `codefidence check-update`.
pub fn check_force() -> Result<String> {
    let latest = fetch_latest_version()?;

    if is_newer(&latest, CURRENT_VERSION) {
        Ok(format_update_message(CURRENT_VERSION, &latest))
    } else {
        Ok(format!(
            "[codefidence] You are up to date (v{}).",
            CURRENT_VERSION
        ))
    }
}

/// Append an update warning to hook context parts (if outdated).
pub fn append_to_hook_context(context_parts: &mut Vec<String>) {
    if let Some(msg) = check_background() {
        context_parts.push(msg);
    }
}

// ─── Internals ───

fn check_inner(force: bool) -> Result<Option<String>> {
    // Try cached value first
    if !force {
        if let Some(cache) = read_cache() {
            if let Ok(checked_at) = cache.checked_at.parse::<DateTime<Utc>>() {
                let age = Utc::now() - checked_at;
                if age.num_hours() < CACHE_TTL_HOURS {
                    return Ok(if is_newer(&cache.latest_version, CURRENT_VERSION) {
                        Some(format_update_message(
                            CURRENT_VERSION,
                            &cache.latest_version,
                        ))
                    } else {
                        None
                    });
                }
            }
        }
    }

    // Cache expired or missing — fetch from crates.io
    let latest = fetch_latest_version()?;
    write_cache(&latest);

    Ok(if is_newer(&latest, CURRENT_VERSION) {
        Some(format_update_message(CURRENT_VERSION, &latest))
    } else {
        None
    })
}

fn fetch_latest_version() -> Result<String> {
    let url = format!("https://crates.io/api/v1/crates/{}", CRATE_NAME);

    let agent = ureq::AgentBuilder::new()
        .timeout(Duration::from_secs(HTTP_TIMEOUT_SECS))
        .user_agent(&format!(
            "{}/{} (update-check)",
            CRATE_NAME, CURRENT_VERSION
        ))
        .build();

    let body = agent
        .get(&url)
        .call()
        .context("Failed to reach crates.io")?
        .into_string()
        .context("Failed to read crates.io response")?;

    let response: CratesIoResponse =
        serde_json::from_str(&body).context("Failed to parse crates.io response")?;

    Ok(response.krate.max_version)
}

/// Compare two semver strings. Returns true if `latest` is strictly newer than `current`.
fn is_newer(latest: &str, current: &str) -> bool {
    let parse = |v: &str| -> Vec<u64> {
        v.split('.')
            .filter_map(|part| part.parse::<u64>().ok())
            .collect()
    };

    let l = parse(latest);
    let c = parse(current);

    // Compare component by component
    for i in 0..l.len().max(c.len()) {
        let lv = l.get(i).copied().unwrap_or(0);
        let cv = c.get(i).copied().unwrap_or(0);
        if lv > cv {
            return true;
        }
        if lv < cv {
            return false;
        }
    }

    false
}

fn format_update_message(current: &str, latest: &str) -> String {
    format!(
        "[codefidence] Update available: {} \u{2192} {}\n  \
         npm:   npm update -g @agence-debord/codefidence\n  \
         brew:  brew upgrade codefidence\n  \
         cargo: cargo install codefidence",
        current, latest
    )
}

// ─── Cache I/O ───

fn cache_dir() -> Option<PathBuf> {
    let base = if let Ok(xdg) = std::env::var("XDG_CACHE_HOME") {
        PathBuf::from(xdg)
    } else {
        dirs_cache_fallback()?
    };

    Some(base.join("codefidence"))
}

/// Fallback: ~/.cache on Unix, %LOCALAPPDATA% on Windows
fn dirs_cache_fallback() -> Option<PathBuf> {
    #[cfg(unix)]
    {
        std::env::var("HOME")
            .ok()
            .map(|h| PathBuf::from(h).join(".cache"))
    }

    #[cfg(windows)]
    {
        std::env::var("LOCALAPPDATA").ok().map(PathBuf::from)
    }
}

fn cache_path() -> Option<PathBuf> {
    cache_dir().map(|d| d.join("update-check.json"))
}

fn read_cache() -> Option<UpdateCache> {
    let path = cache_path()?;
    let content = fs::read_to_string(path).ok()?;
    serde_json::from_str(&content).ok()
}

fn write_cache(latest_version: &str) {
    let Some(path) = cache_path() else { return };

    if let Some(parent) = path.parent() {
        let _ = fs::create_dir_all(parent);
    }

    let cache = UpdateCache {
        latest_version: latest_version.to_string(),
        checked_at: Utc::now().to_rfc3339(),
    };

    if let Ok(json) = serde_json::to_string_pretty(&cache) {
        let _ = fs::write(path, json);
    }
}

// ─── Tests ───

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_newer_basic() {
        assert!(is_newer("0.4.0", "0.3.3"));
        assert!(is_newer("1.0.0", "0.9.9"));
        assert!(is_newer("0.3.4", "0.3.3"));
    }

    #[test]
    fn is_newer_equal() {
        assert!(!is_newer("0.3.3", "0.3.3"));
        assert!(!is_newer("1.0.0", "1.0.0"));
    }

    #[test]
    fn is_newer_older() {
        assert!(!is_newer("0.3.2", "0.3.3"));
        assert!(!is_newer("0.2.9", "0.3.0"));
    }

    #[test]
    fn is_newer_different_lengths() {
        assert!(is_newer("0.3.3.1", "0.3.3"));
        assert!(!is_newer("0.3.3", "0.3.3.1"));
    }

    #[test]
    fn format_message_contains_versions() {
        let msg = format_update_message("0.3.3", "0.4.0");
        assert!(msg.contains("0.3.3"));
        assert!(msg.contains("0.4.0"));
        assert!(msg.contains("npm"));
        assert!(msg.contains("brew"));
        assert!(msg.contains("cargo"));
    }
}
