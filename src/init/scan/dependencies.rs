use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};

use super::imports::FileImports;
use super::structure::extract_domain_name;

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
        let source_domain = match extract_domain_name(&fi.file_path, root) {
            Some(d) => d,
            None => continue,
        };

        if !domains.contains_key(&source_domain) {
            continue;
        }

        for import_path in &fi.imports {
            let import_lower = import_path.to_lowercase().replace('_', "-");

            for (target_domain, idents) in &domain_identifiers {
                if *target_domain == source_domain {
                    continue;
                }

                // Check if any identifier matches a portion of the import path
                let matches = idents.iter().any(|ident| {
                    let ident_lower = ident.to_lowercase().replace('_', "-");
                    import_lower.contains(&ident_lower) || ident_lower.contains(&import_lower)
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
