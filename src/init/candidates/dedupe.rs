use std::collections::HashSet;

use super::{Candidate, CandidateType, MAX_CANDIDATES};

pub(super) fn type_priority(t: &CandidateType) -> u8 {
    match t {
        CandidateType::Exception => 0,
        CandidateType::Decision => 1,
        CandidateType::BusinessRule => 2,
    }
}

pub(super) fn deduplicate(candidates: &mut Vec<Candidate>) {
    let mut seen: HashSet<(String, String)> = HashSet::new(); // (main_file, type)
    candidates.retain(|c| {
        let main_file = c
            .provenance
            .first()
            .map(|p| p.ref_.clone())
            .unwrap_or_default();
        let key = (main_file, c.type_.to_string());
        seen.insert(key)
    });
}

pub(super) fn prioritize_and_limit(candidates: &mut Vec<Candidate>) {
    // Sort: exception > decision > business_rule, then by provenance richness
    candidates.sort_by(|a, b| {
        let type_cmp = type_priority(&a.type_).cmp(&type_priority(&b.type_));
        if type_cmp != std::cmp::Ordering::Equal {
            return type_cmp;
        }
        b.provenance.len().cmp(&a.provenance.len())
    });

    candidates.truncate(MAX_CANDIDATES);
}

pub(super) fn assign_ids(candidates: &mut [Candidate]) {
    let mut domain_counters: std::collections::HashMap<String, u32> =
        std::collections::HashMap::new();

    for c in candidates.iter_mut() {
        let counter = domain_counters.entry(c.domain.clone()).or_insert(0);
        *counter += 1;
        c.id = format!("{}-{:03}", c.domain, counter);
    }
}
