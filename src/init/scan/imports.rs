use std::path::{Path, PathBuf};
use std::sync::LazyLock;

use rayon::prelude::*;
use regex::Regex;

// ─── Pre-compiled regex patterns ───

// JS/TS imports
static RE_JS_IMPORT: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r#"import\s+.*?\s+from\s+['"]([^'"]+)['"]"#).unwrap());
static RE_JS_REQUIRE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r#"require\(\s*['"]([^'"]+)['"]\s*\)"#).unwrap());
static RE_JS_EXPORT: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r#"export\s+.*?\s+from\s+['"]([^'"]+)['"]"#).unwrap());

// Python imports
static RE_PY_FROM: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^from\s+(\S+)\s+import").unwrap());
static RE_PY_IMPORT: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^import\s+(\S+)").unwrap());

// Rust imports
static RE_RS_USE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"use\s+crate::(\S+?)(?:::\{|;)").unwrap());
static RE_RS_MOD: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^(?:pub\s+)?mod\s+(\w+)\s*;").unwrap());

// Go imports
static RE_GO_SINGLE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r#"import\s+"([^"]+)""#).unwrap());
static RE_GO_BLOCK: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r#"import\s*\(([\s\S]*?)\)"#).unwrap());
static RE_GO_PATH: LazyLock<Regex> = LazyLock::new(|| Regex::new(r#""([^"]+)""#).unwrap());

// ─── Public types ───

#[derive(Debug, Default)]
pub struct FileImports {
    pub file_path: PathBuf,
    pub imports: Vec<String>,
}

// ─── Import extraction ───

pub fn extract_all_imports(files: &[&PathBuf], _root: &Path) -> Vec<FileImports> {
    files
        .par_iter()
        .filter_map(|path| {
            let content = std::fs::read_to_string(path).ok()?;
            let imports = extract_imports(path, &content);
            if imports.is_empty() {
                None
            } else {
                Some(FileImports {
                    file_path: path.to_path_buf(),
                    imports,
                })
            }
        })
        .collect()
}

fn extract_imports(path: &Path, content: &str) -> Vec<String> {
    let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("");

    match ext {
        "ts" | "tsx" | "js" | "jsx" => extract_js_imports(content),
        "py" => extract_python_imports(content),
        "rs" => extract_rust_imports(content),
        "go" => extract_go_imports(content),
        _ => Vec::new(),
    }
}

fn extract_js_imports(content: &str) -> Vec<String> {
    let mut imports = Vec::new();

    // import ... from '...'
    for cap in RE_JS_IMPORT.captures_iter(content) {
        imports.push(cap[1].to_string());
    }

    // require('...')
    for cap in RE_JS_REQUIRE.captures_iter(content) {
        imports.push(cap[1].to_string());
    }

    // export ... from '...'
    for cap in RE_JS_EXPORT.captures_iter(content) {
        imports.push(cap[1].to_string());
    }

    imports
}

fn extract_python_imports(content: &str) -> Vec<String> {
    let mut imports = Vec::new();

    // from X import Y
    for line in content.lines() {
        let trimmed = line.trim();
        if let Some(cap) = RE_PY_FROM.captures(trimmed) {
            imports.push(cap[1].to_string());
        }
    }

    // import X
    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("import ") && !trimmed.contains(" from ") {
            if let Some(cap) = RE_PY_IMPORT.captures(trimmed) {
                imports.push(cap[1].to_string());
            }
        }
    }

    imports
}

fn extract_rust_imports(content: &str) -> Vec<String> {
    let mut imports = Vec::new();

    // use crate::something
    for cap in RE_RS_USE.captures_iter(content) {
        imports.push(cap[1].to_string());
    }

    // mod something
    for line in content.lines() {
        let trimmed = line.trim();
        if let Some(cap) = RE_RS_MOD.captures(trimmed) {
            imports.push(cap[1].to_string());
        }
    }

    imports
}

fn extract_go_imports(content: &str) -> Vec<String> {
    let mut imports = Vec::new();

    // Single import: import "path"
    for cap in RE_GO_SINGLE.captures_iter(content) {
        imports.push(cap[1].to_string());
    }

    // Multi-line import block: import ( "path1" "path2" )
    for block_cap in RE_GO_BLOCK.captures_iter(content) {
        for path_cap in RE_GO_PATH.captures_iter(&block_cap[1]) {
            imports.push(path_cap[1].to_string());
        }
    }

    imports
}
