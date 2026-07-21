use cuecode_plans::{
    ArtifactKind, ArtifactRoot, ArtifactStatus, ProjectManifestFile, count_checkboxes,
    validate_manifest, validate_manifest_on_disk,
};
use pretty_assertions::assert_eq;
use std::path::Path;

#[test]
fn counts_task_checkboxes() {
    let source = r#"
## Tasks
- [ ] one
- [x] two
- [X] three
- not a checkbox
"#;
    let progress = count_checkboxes(source);
    assert_eq!(progress.checked, 2);
    assert_eq!(progress.total, 3);
    assert!(!progress.all_checked());
}

#[test]
fn validates_dogfood_fixture_when_present() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../..")
        .canonicalize()
        .expect("repo root");
    let manifest_path = root.join(".cuecode/project.yaml");
    if !manifest_path.is_file() {
        return;
    }
    let loaded = cuecode_plans::load_manifest_from_disk(&root)
        .expect("load")
        .expect("manifest");
    let report = validate_manifest_on_disk(&root, &loaded.manifest);
    assert!(
        report.is_valid(),
        "dogfood manifest invalid: {:?}",
        report.errors
    );
}

#[test]
fn rejects_duplicate_artifact_ids() {
    let manifest = ProjectManifestFile {
        version: 1,
        project: cuecode_plans::ProjectInfo {
            name: "test".into(),
            adopted_at: "2026-06-19".into(),
        },
        roots: Default::default(),
        artifacts: vec![
            cuecode_plans::Artifact {
                id: "dup".into(),
                kind: ArtifactKind::BuildPhase,
                path: "a.md".into(),
                root: ArtifactRoot::External,
                canonical: false,
                status: ArtifactStatus::NotStarted,
                depends_on: vec![],
                blocks: vec![],
                pin_policy: None,
                refs: vec![],
            },
            cuecode_plans::Artifact {
                id: "dup".into(),
                kind: ArtifactKind::ProductPlan,
                path: "b.md".into(),
                root: ArtifactRoot::External,
                canonical: false,
                status: ArtifactStatus::NotStarted,
                depends_on: vec![],
                blocks: vec![],
                pin_policy: None,
                refs: vec![],
            },
        ],
        build_track: Default::default(),
        docs: Default::default(),
    };
    let report = validate_manifest(&manifest);
    assert!(!report.is_valid());
}
