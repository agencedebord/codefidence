use std::path::{Path, PathBuf};
use std::sync::LazyLock;

use rayon::prelude::*;
use regex::Regex;

use super::structure::is_source_file;

// ─── Pre-compiled regex patterns ───

// Comments (TODO/FIXME/HACK/NOTE)
static RE_COMMENTS: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(?://|#|/\*)\s*(TODO|FIXME|HACK|NOTE)\b[:\s]*(.*)").unwrap());

// Model/type definitions per language
static RE_JS_MODELS: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(?:export\s+)?(?:interface|type|class|enum)\s+(\w+)").unwrap());
static RE_PY_CLASS: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"class\s+(\w+)").unwrap());
static RE_RS_STRUCT: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(?:pub\s+)?(?:struct|enum|trait)\s+(\w+)").unwrap());
static RE_GO_TYPE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"type\s+(\w+)\s+struct").unwrap());

// Route/endpoint extraction
static RE_EXPRESS: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"(?:app|router)\.\s*(get|post|put|patch|delete)\s*\(\s*['"]([^'"]+)['"]"#).unwrap()
});
static RE_FLASK: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"@\w+\.(?:route|get|post|put|patch|delete)\s*\(\s*['"]([^'"]+)['"]"#).unwrap()
});
static RE_NEXTJS: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"export\s+(?:async\s+)?function\s+(GET|POST|PUT|PATCH|DELETE)").unwrap()
});
static RE_ACTIX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r#"#\[\s*(get|post|put|patch|delete)\s*\(\s*"([^"]+)""#).unwrap());
static RE_GO_HTTP: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r#"(?:HandleFunc|Handle)\s*\(\s*"([^"]+)""#).unwrap());

// ─── Types ───

#[derive(Debug, Default)]
pub struct DomainDetails {
    pub models: Vec<String>,
    pub routes: Vec<String>,
    pub comments: Vec<String>,
}

// ─── Detail extraction ───

pub fn extract_details(files: &[PathBuf], _root: &Path) -> DomainDetails {
    let results: Vec<DomainDetails> = files
        .par_iter()
        .filter_map(|path| {
            if !is_source_file(path) {
                return None;
            }
            let content = std::fs::read_to_string(path).ok()?;
            Some(extract_file_details(&content, path))
        })
        .collect();

    let mut merged = DomainDetails::default();
    for r in results {
        merged.models.extend(r.models);
        merged.routes.extend(r.routes);
        merged.comments.extend(r.comments);
    }

    // Deduplicate
    merged.models.sort();
    merged.models.dedup();
    merged.routes.sort();
    merged.routes.dedup();

    merged
}

fn extract_file_details(content: &str, path: &Path) -> DomainDetails {
    let mut details = DomainDetails::default();

    let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("");

    // Extract TODO/FIXME/HACK/NOTE comments
    for cap in RE_COMMENTS.captures_iter(content) {
        let tag = &cap[1];
        let text = cap[2].trim().trim_end_matches("*/").trim();
        if !text.is_empty() {
            details.comments.push(format!("[{}] {}", tag, text));
        }
    }

    // Extract model/type/struct/class/interface definitions
    extract_models(content, ext, &mut details);

    // Extract route/endpoint definitions
    extract_routes(content, ext, path, &mut details);

    details
}

fn extract_models(content: &str, ext: &str, details: &mut DomainDetails) {
    match ext {
        "ts" | "tsx" | "js" | "jsx" => {
            for cap in RE_JS_MODELS.captures_iter(content) {
                details.models.push(cap[1].to_string());
            }
        }
        "py" => {
            for cap in RE_PY_CLASS.captures_iter(content) {
                details.models.push(cap[1].to_string());
            }
        }
        "rs" => {
            for cap in RE_RS_STRUCT.captures_iter(content) {
                details.models.push(cap[1].to_string());
            }
        }
        "go" => {
            for cap in RE_GO_TYPE.captures_iter(content) {
                details.models.push(cap[1].to_string());
            }
        }
        _ => {}
    }
}

fn extract_routes(content: &str, ext: &str, path: &Path, details: &mut DomainDetails) {
    // Express-style: app.get('/...'), router.post('/...')
    for cap in RE_EXPRESS.captures_iter(content) {
        details
            .routes
            .push(format!("{} {}", cap[1].to_uppercase(), &cap[2]));
    }

    // Python/Flask/FastAPI decorators: @app.route('/...'), @router.get('/...')
    for cap in RE_FLASK.captures_iter(content) {
        details.routes.push(cap[1].to_string());
    }

    // Next.js API routes (infer from file path pattern)
    let path_str = path.to_string_lossy();
    if path_str.contains("/api/") && (ext == "ts" || ext == "js" || ext == "tsx" || ext == "jsx") {
        // Check for HTTP method exports: export async function GET/POST/etc.
        for cap in RE_NEXTJS.captures_iter(content) {
            if let Some(route) = extract_nextjs_route(path) {
                details.routes.push(format!("{} {}", &cap[1], route));
            }
        }
    }

    // Rust Actix/Axum style: #[get("/...")]
    for cap in RE_ACTIX.captures_iter(content) {
        details
            .routes
            .push(format!("{} {}", cap[1].to_uppercase(), &cap[2]));
    }

    // Go: http.HandleFunc("/...", handler)
    for cap in RE_GO_HTTP.captures_iter(content) {
        details.routes.push(cap[1].to_string());
    }
}

fn extract_nextjs_route(path: &Path) -> Option<String> {
    let path_str = path.to_string_lossy();
    // Find the /api/ segment and build route from it
    if let Some(idx) = path_str.find("/api/") {
        let route_part = &path_str[idx..];
        // Remove file extension and route.ts/route.js
        let route = route_part
            .trim_end_matches(".ts")
            .trim_end_matches(".tsx")
            .trim_end_matches(".js")
            .trim_end_matches(".jsx")
            .trim_end_matches("/route")
            .trim_end_matches("/index");
        return Some(route.to_string());
    }
    None
}
