use super::prioritize::prioritize_memory_items;
use super::render::{compact_summary, extract_bullet_points, extract_sections};
use super::MAX_CONTEXT_LEN;
use crate::wiki::note::{
    Confidence, MemoryItem, MemoryItemSource, MemoryItemStatus, MemoryItemType, WikiNote,
};
use chrono::NaiveDate;

fn make_note(domain: &str, confidence: Confidence, content: &str) -> WikiNote {
    WikiNote {
        path: format!(".wiki/domains/{}/_overview.md", domain),
        domain: domain.to_string(),
        confidence,
        last_updated: Some(NaiveDate::from_ymd_opt(2026, 3, 28).unwrap()),
        related_files: vec![format!("src/{}/main.ts", domain)],
        deprecated: false,
        title: format!("{} overview", domain),
        content: content.to_string(),
        memory_items: Vec::new(),
    }
}

fn make_item(
    id: &str,
    type_: MemoryItemType,
    text: &str,
    confidence: Confidence,
) -> MemoryItem {
    MemoryItem {
        id: id.to_string(),
        type_,
        text: text.to_string(),
        confidence,
        related_files: Vec::new(),
        sources: vec![MemoryItemSource {
            kind: "file".to_string(),
            ref_: "src/test.ts".to_string(),
            line: None,
        }],
        status: MemoryItemStatus::Active,
        last_reviewed: None,
    }
}

fn make_note_with_items(
    domain: &str,
    confidence: Confidence,
    items: Vec<MemoryItem>,
) -> WikiNote {
    WikiNote {
        path: format!(".wiki/domains/{}/_overview.md", domain),
        domain: domain.to_string(),
        confidence,
        last_updated: Some(NaiveDate::from_ymd_opt(2026, 3, 28).unwrap()),
        related_files: vec![format!("src/{}/main.ts", domain)],
        deprecated: false,
        title: format!("{} overview", domain),
        content: "## Dependencies\n- payments\n- taxes\n".to_string(),
        memory_items: items,
    }
}

// ─── Fallback tests (notes without memory_items) ───

#[test]
fn compact_summary_includes_domain_info() {
    let note = make_note(
        "billing",
        Confidence::Confirmed,
        "## Key behaviors\n- Generates invoices\n- Handles refunds\n",
    );
    let summary = compact_summary(&note, "billing", "src/billing/main.ts");

    assert!(summary.contains("[project-wiki] Domain: billing"));
    assert!(summary.contains("confirmed"));
    assert!(summary.contains("2026-03-28"));
}

#[test]
fn compact_summary_includes_behaviors() {
    let note = make_note(
        "billing",
        Confidence::Confirmed,
        "## Key behaviors\n- Generates invoices\n- Handles refunds\n",
    );
    let summary = compact_summary(&note, "billing", "src/billing/main.ts");

    assert!(summary.contains("Generates invoices"));
    assert!(summary.contains("Handles refunds"));
}

#[test]
fn compact_summary_includes_business_rules() {
    let note = make_note(
        "billing",
        Confidence::Confirmed,
        "## Business rules\n- No dedup on import\n",
    );
    let summary = compact_summary(&note, "billing", "src/billing/main.ts");

    assert!(summary.contains("No dedup on import"));
}

#[test]
fn compact_summary_includes_related_files() {
    let note = make_note("billing", Confidence::Confirmed, "# Billing\n");
    let summary = compact_summary(&note, "billing", "src/billing/main.ts");

    assert!(summary.contains("src/billing/main.ts"));
}

#[test]
fn compact_summary_warns_on_low_confidence() {
    let note = make_note("billing", Confidence::NeedsValidation, "# Billing\n");
    let summary = compact_summary(&note, "billing", "src/billing/main.ts");

    assert!(summary.contains("WARNING"));
    assert!(summary.contains("needs-validation"));
}

#[test]
fn compact_summary_no_warning_on_confirmed() {
    let note = make_note("billing", Confidence::Confirmed, "# Billing\n");
    let summary = compact_summary(&note, "billing", "src/billing/main.ts");

    assert!(!summary.contains("WARNING"));
}

#[test]
fn compact_summary_truncates_long_content() {
    let mut long_content = String::new();
    for section in &[
        "Key behaviors",
        "Business rules",
        "Dependencies",
        "Architecture notes",
    ] {
        long_content.push_str(&format!("## {}\n", section));
        for i in 0..50 {
            long_content.push_str(&format!(
                "- Item {} in {} with a very long description that adds significant length to the output to ensure we exceed the truncation threshold eventually\n",
                i, section
            ));
        }
    }
    let mut note = make_note("billing", Confidence::Confirmed, &long_content);
    note.related_files = (0..50)
        .map(|i| format!("src/billing/very/deep/nested/module_{}/handler.ts", i))
        .collect();
    let summary = compact_summary(&note, "billing", "src/billing/main.ts");

    assert!(summary.len() <= MAX_CONTEXT_LEN);
    assert!(summary.contains("[... truncated]"));
}

#[test]
fn extract_sections_parses_headings() {
    let content = "## Description\nSome text.\n\n## Key behaviors\n- One\n- Two\n";
    let sections = extract_sections(content);

    assert!(sections.contains_key("description"));
    assert!(sections.contains_key("key behaviors"));
    assert!(sections["key behaviors"].contains("- One"));
}

#[test]
fn extract_bullet_points_limits_count() {
    let body = "- One\n- Two\n- Three\n- Four\n- Five\n- Six\n";
    let items = extract_bullet_points(body, 3);
    assert_eq!(items.len(), 3);
}

#[test]
fn extract_bullet_points_skips_placeholders() {
    let body = "- _None detected._\n- Real item\n";
    let items = extract_bullet_points(body, 10);
    assert_eq!(items.len(), 1);
    assert_eq!(items[0], "Real item");
}

// ─── V1 tests (notes with memory_items) ───

#[test]
fn context_v1_prioritize_exception_first() {
    let items = vec![
        make_item(
            "b-001",
            MemoryItemType::BusinessRule,
            "Rule A",
            Confidence::Confirmed,
        ),
        make_item(
            "b-002",
            MemoryItemType::Decision,
            "Decision B",
            Confidence::Confirmed,
        ),
        make_item(
            "b-003",
            MemoryItemType::Exception,
            "Exception C",
            Confidence::Confirmed,
        ),
    ];
    let note = make_note_with_items("billing", Confidence::Confirmed, items);
    let summary = compact_summary(&note, "billing", "src/billing/main.ts");

    let exc_pos = summary.find("[exception]").unwrap();
    let dec_pos = summary.find("[decision]").unwrap();
    let rule_pos = summary.find("[business_rule]").unwrap();
    assert!(exc_pos < dec_pos, "exception should come before decision");
    assert!(
        dec_pos < rule_pos,
        "decision should come before business_rule"
    );
}

#[test]
fn context_v1_secondary_sort_by_confidence() {
    let items = vec![
        make_item(
            "b-001",
            MemoryItemType::Decision,
            "Inferred decision",
            Confidence::Inferred,
        ),
        make_item(
            "b-002",
            MemoryItemType::Decision,
            "Confirmed decision",
            Confidence::Confirmed,
        ),
    ];
    let note = make_note_with_items("billing", Confidence::Confirmed, items);
    let summary = compact_summary(&note, "billing", "src/billing/main.ts");

    let confirmed_pos = summary.find("Confirmed decision").unwrap();
    let inferred_pos = summary.find("Inferred decision").unwrap();
    assert!(
        confirmed_pos < inferred_pos,
        "confirmed should come before inferred"
    );
}

#[test]
fn context_v1_secondary_sort_by_related_file() {
    let mut item_related = make_item(
        "b-001",
        MemoryItemType::Decision,
        "Related decision",
        Confidence::Confirmed,
    );
    item_related.related_files = vec!["src/billing/invoice.ts".to_string()];

    let item_unrelated = make_item(
        "b-002",
        MemoryItemType::Decision,
        "Unrelated decision",
        Confidence::Confirmed,
    );

    let note = make_note_with_items(
        "billing",
        Confidence::Confirmed,
        vec![item_unrelated, item_related],
    );
    let summary = compact_summary(&note, "billing", "src/billing/invoice.ts");

    let related_pos = summary.find("Related decision").unwrap();
    let unrelated_pos = summary.find("Unrelated decision").unwrap();
    assert!(
        related_pos < unrelated_pos,
        "related file match should come first"
    );
}

#[test]
fn context_v1_limit_3_items() {
    let items = vec![
        make_item(
            "b-001",
            MemoryItemType::Exception,
            "Exc 1",
            Confidence::Confirmed,
        ),
        make_item(
            "b-002",
            MemoryItemType::Decision,
            "Dec 2",
            Confidence::Confirmed,
        ),
        make_item(
            "b-003",
            MemoryItemType::BusinessRule,
            "Rule 3",
            Confidence::Confirmed,
        ),
        make_item(
            "b-004",
            MemoryItemType::BusinessRule,
            "Rule 4",
            Confidence::Confirmed,
        ),
        make_item(
            "b-005",
            MemoryItemType::BusinessRule,
            "Rule 5",
            Confidence::Confirmed,
        ),
    ];
    let note = make_note_with_items("billing", Confidence::Confirmed, items);
    let summary = compact_summary(&note, "billing", "src/billing/main.ts");

    assert!(summary.contains("Exc 1"));
    assert!(summary.contains("Dec 2"));
    assert!(summary.contains("Rule 3"));
    assert!(!summary.contains("Rule 4"));
    assert!(!summary.contains("Rule 5"));
    assert!(summary.contains("(+2 more items)"));
}

#[test]
fn context_v1_format_type_and_confidence_brackets() {
    let items = vec![make_item(
        "b-001",
        MemoryItemType::Exception,
        "Client X uses old calc",
        Confidence::Confirmed,
    )];
    let note = make_note_with_items("billing", Confidence::Confirmed, items);
    let summary = compact_summary(&note, "billing", "src/billing/main.ts");

    assert!(summary.contains("[exception] Client X uses old calc [confirmed]"));
}

#[test]
fn context_v1_warning_low_confidence_items() {
    let items = vec![
        make_item(
            "b-001",
            MemoryItemType::Decision,
            "Dec A",
            Confidence::Confirmed,
        ),
        make_item(
            "b-002",
            MemoryItemType::BusinessRule,
            "Rule B",
            Confidence::Inferred,
        ),
    ];
    let note = make_note_with_items("billing", Confidence::Confirmed, items);
    let summary = compact_summary(&note, "billing", "src/billing/main.ts");

    assert!(summary.contains("WARNING"));
    assert!(summary.contains("1 item(s) have low confidence"));
}

#[test]
fn context_v1_no_warning_all_confirmed() {
    let items = vec![make_item(
        "b-001",
        MemoryItemType::Decision,
        "Dec A",
        Confidence::Confirmed,
    )];
    let note = make_note_with_items("billing", Confidence::Confirmed, items);
    let summary = compact_summary(&note, "billing", "src/billing/main.ts");

    assert!(!summary.contains("WARNING"));
}

#[test]
fn context_v1_includes_dependencies() {
    let items = vec![make_item(
        "b-001",
        MemoryItemType::Decision,
        "Dec A",
        Confidence::Confirmed,
    )];
    let note = make_note_with_items("billing", Confidence::Confirmed, items);
    let summary = compact_summary(&note, "billing", "src/billing/main.ts");

    assert!(summary.contains("Dependencies: payments, taxes"));
}

#[test]
fn context_v1_filters_deprecated_items() {
    let mut deprecated_item = make_item(
        "b-001",
        MemoryItemType::Exception,
        "Old exception",
        Confidence::Confirmed,
    );
    deprecated_item.status = MemoryItemStatus::Deprecated;

    let active_item = make_item(
        "b-002",
        MemoryItemType::Decision,
        "Active decision",
        Confidence::Confirmed,
    );

    let note = make_note_with_items(
        "billing",
        Confidence::Confirmed,
        vec![deprecated_item, active_item],
    );
    let summary = compact_summary(&note, "billing", "src/billing/main.ts");

    assert!(!summary.contains("Old exception"));
    assert!(summary.contains("Active decision"));
}

#[test]
fn context_fallback_when_no_memory_items() {
    let note = make_note(
        "billing",
        Confidence::Confirmed,
        "## Key behaviors\n- Generates invoices\n## Business rules\n- No dedup\n",
    );
    let summary = compact_summary(&note, "billing", "src/billing/main.ts");

    // Fallback should show markdown sections, not "Memory:" header
    assert!(!summary.contains("Memory:"));
    assert!(summary.contains("Key behaviors:"));
    assert!(summary.contains("Business rules:"));
}

// ─── Prioritization unit tests ───

#[test]
fn prioritize_respects_type_order() {
    let items = vec![
        make_item(
            "1",
            MemoryItemType::BusinessRule,
            "Rule",
            Confidence::Confirmed,
        ),
        make_item("2", MemoryItemType::Exception, "Exc", Confidence::Confirmed),
        make_item("3", MemoryItemType::Decision, "Dec", Confidence::Confirmed),
    ];

    let result = prioritize_memory_items(&items, "", 3);
    assert_eq!(result[0].type_, MemoryItemType::Exception);
    assert_eq!(result[1].type_, MemoryItemType::Decision);
    assert_eq!(result[2].type_, MemoryItemType::BusinessRule);
}

#[test]
fn prioritize_filters_deprecated() {
    let mut dep = make_item("1", MemoryItemType::Exception, "Old", Confidence::Confirmed);
    dep.status = MemoryItemStatus::Deprecated;
    let active = make_item("2", MemoryItemType::Decision, "New", Confidence::Confirmed);

    let items = vec![dep, active];
    let result = prioritize_memory_items(&items, "", 3);
    assert_eq!(result.len(), 1);
    assert_eq!(result[0].id, "2");
}

#[test]
fn prioritize_respects_max() {
    let items: Vec<MemoryItem> = (0..10)
        .map(|i| {
            make_item(
                &format!("b-{:03}", i),
                MemoryItemType::BusinessRule,
                &format!("Rule {}", i),
                Confidence::Confirmed,
            )
        })
        .collect();

    let result = prioritize_memory_items(&items, "", 3);
    assert_eq!(result.len(), 3);
}
