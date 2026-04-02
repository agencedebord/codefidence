use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};

use super::imports::FileImports;

/// Build a dependency graph from file imports and domain assignments.
///
/// For each file with imports, find which domain it belongs to,
/// then check if its imports point to other domains.
pub fn build_dependency_graph(
    domains: &HashMap<String, Vec<PathBuf>>,
    imports: &[FileImports],
    root: &Path,
) -> HashMap<String, Vec<String>> {
    let mut graph: HashMap<String, Vec<String>> = HashMap::new();

    // Build a reverse lookup: file path -> domain name.
    // This uses the already-computed domain assignments (which include app-dir detection)
    // instead of re-running extract_domain_name() which may not know about app dirs.
    let mut file_to_domain: HashMap<PathBuf, String> = HashMap::new();
    for (name, files) in domains {
        for file in files {
            file_to_domain.insert(file.clone(), name.clone());
        }
    }

    // Build a lookup: domain name -> set of path fragments that identify this domain
    let mut domain_identifiers: HashMap<String, HashSet<String>> = HashMap::new();
    for (name, files) in domains {
        let mut idents = HashSet::new();
        idents.insert(name.clone());
        idents.insert(name.replace('-', "_"));

        for file in files {
            if let Ok(rel) = file.strip_prefix(root) {
                // Add the relative path so we can match imports against it
                let rel_str = rel.to_string_lossy().to_string();
                idents.insert(rel_str);

                // Also add without extension
                if let Some(stem) = rel.with_extension("").to_str() {
                    idents.insert(stem.to_string());
                }
            }
        }

        domain_identifiers.insert(name.clone(), idents);
    }

    for fi in imports {
        // Use the pre-computed file→domain mapping
        let source_domain = match file_to_domain.get(&fi.file_path) {
            Some(d) => d.clone(),
            None => continue,
        };

        for import_path in &fi.imports {
            let import_lower = import_path.to_lowercase().replace('_', "-");

            for (target_domain, idents) in &domain_identifiers {
                if *target_domain == source_domain {
                    continue;
                }

                // Check if any identifier matches as a path component of the import.
                // We use component-based matching (split by '.' or '/') to avoid
                // false positives like "django.apps" matching domain "app".
                let matches = idents.iter().any(|ident| {
                    let ident_lower = ident.to_lowercase().replace('_', "-");
                    import_matches_ident(&import_lower, &ident_lower)
                });

                if matches {
                    let deps = graph.entry(source_domain.clone()).or_default();
                    if !deps.contains(target_domain) {
                        deps.push(target_domain.clone());
                    }
                }
            }
        }
    }

    // Sort dependency lists
    for deps in graph.values_mut() {
        deps.sort();
    }

    graph
}

/// Check if an import path matches a domain identifier using component-based matching.
///
/// We normalize both paths by treating `.` and `/` as separators, then check
/// if the import starts with the identifier as a path prefix.
/// This avoids false positives like "django.apps" matching domain "app".
///
/// Examples:
/// - "accounts.models" matches "accounts" ✅ (starts with "accounts.")
/// - "django.apps" does NOT match "app" ✅ (no prefix match)
/// - "app.decorators" matches "app" ✅ (starts with "app.")
/// - "lib.utils" matches "lib" ✅ (starts with "lib.")
/// - "accounts/models" matches "accounts/models" ✅ (exact)
fn import_matches_ident(import_lower: &str, ident_lower: &str) -> bool {
    // Normalize separators: treat '.' and '/' uniformly
    let import_norm = import_lower.replace('.', "/");
    let ident_norm = ident_lower.replace('.', "/");

    // Exact match
    if import_norm == ident_norm {
        return true;
    }

    // Import starts with ident as a path prefix (ident + "/")
    if import_norm.starts_with(&format!("{}/", ident_norm)) {
        return true;
    }

    // Ident starts with import as a path prefix (import + "/")
    // Handles cases where import is "accounts" and ident is "accounts/models"
    if ident_norm.starts_with(&format!("{}/", import_norm)) {
        return true;
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    // ─── import_matches_ident ───

    #[test]
    fn matches_exact_domain_name() {
        assert!(import_matches_ident("accounts", "accounts"));
    }

    #[test]
    fn matches_python_import_to_domain() {
        // from accounts.models import Profile
        assert!(import_matches_ident("accounts.models", "accounts"));
    }

    #[test]
    fn no_false_positive_django_apps() {
        // "django.apps" should NOT match domain "app"
        assert!(!import_matches_ident("django.apps", "app"));
    }

    #[test]
    fn no_false_positive_substring() {
        // "application" should NOT match domain "app"
        assert!(!import_matches_ident("application.utils", "app"));
    }

    #[test]
    fn matches_reverse_prefix() {
        // import "accounts" matches ident "accounts/models"
        assert!(import_matches_ident("accounts", "accounts/models"));
    }

    #[test]
    fn matches_with_dot_separator() {
        assert!(import_matches_ident("app.decorators", "app"));
    }

    #[test]
    fn matches_with_underscore_normalization() {
        // office365_sync after replace('_', '-') = office365-sync
        assert!(import_matches_ident(
            "office365-sync.models",
            "office365-sync"
        ));
    }

    #[test]
    fn no_match_unrelated() {
        assert!(!import_matches_ident("billing.models", "accounts"));
    }

    #[test]
    fn no_match_partial_overlap() {
        // "rest_framework" should not match "rest"
        assert!(!import_matches_ident("rest-framework.views", "rest"));
        // unless "rest" is a path prefix... which it's not since "rest-framework" != "rest/"
    }

    #[test]
    fn matches_exact_file_path() {
        assert!(import_matches_ident("accounts/models", "accounts/models"));
    }
}
