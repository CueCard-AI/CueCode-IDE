use std::path::Path;

use cuecode_specs::{resolve_spec_query, scan_spec_index_from_disk, SpecStatus, SPEC_WATCH_DEBOUNCE};
use settings::WorktreeId;

fn fixture_root() -> std::path::PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/specs_tree")
}

#[test]
fn debounce_is_at_most_two_seconds() {
    assert!(SPEC_WATCH_DEBOUNCE.as_secs() <= 2);
}

#[test]
fn scans_fixture_specs_tree() {
    let root = fixture_root();
    let index = scan_spec_index_from_disk(&root, WorktreeId::from_usize(1)).unwrap();
    assert!(index.entry_count() >= 3, "expected at least 3 specs");

    let titles: Vec<_> = index.entries.iter().map(|e| e.title.as_str()).collect();
    assert!(titles.contains(&"Alpha Spec"));
    assert!(titles.contains(&"Vision"));
}

#[test]
fn parses_frontmatter_status_and_anchors() {
    let root = fixture_root();
    let index = scan_spec_index_from_disk(&root, WorktreeId::from_usize(1)).unwrap();
    let alpha = index
        .entries
        .iter()
        .find(|e| e.path.to_string_lossy().contains("01-alpha"))
        .expect("alpha spec");
    assert_eq!(alpha.status, Some(SpecStatus::Active));
    assert!(alpha.tags.contains(&"core".to_string()));
    assert_eq!(alpha.summary.as_deref(), Some("Alpha summary line."));
    assert!(alpha.anchor_ids.contains(&"alpha-body".to_string()));
}

#[test]
fn resolve_spec_query_matches_path_title_and_anchor() {
    let root = fixture_root();
    let index = scan_spec_index_from_disk(&root, WorktreeId::from_usize(1)).unwrap();

    let by_path = resolve_spec_query(&index, "01-alpha");
    assert_eq!(by_path.len(), 1);

    let by_title = resolve_spec_query(&index, "vision");
    assert_eq!(by_title.len(), 1);
    assert_eq!(by_title[0].title, "Vision");

    let by_anchor = resolve_spec_query(&index, "beta-section");
    assert_eq!(by_anchor.len(), 1);
}

#[test]
fn rescan_picks_up_new_spec_file() {
    let temp = tempfile::tempdir().unwrap();
    let root = temp.path();
    let specs = root.join(".cursor/specs");
    std::fs::create_dir_all(&specs).unwrap();
    std::fs::write(specs.join("first.md"), "# First {#first}\n").unwrap();

    let first = scan_spec_index_from_disk(root, WorktreeId::from_usize(2)).unwrap();
    assert_eq!(first.entry_count(), 1);

    std::fs::write(specs.join("second.md"), "# Second {#second}\n").unwrap();
    let second = scan_spec_index_from_disk(root, WorktreeId::from_usize(2)).unwrap();
    assert_eq!(second.entry_count(), 2);
}
