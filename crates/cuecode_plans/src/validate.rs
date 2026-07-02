use std::path::Path;

use anyhow::{Context as _, Result, bail, ensure};

use crate::manifest::{
    ArtifactKind, ArtifactStatus, LoadedManifest, ProjectManifestFile, find_manifest_path,
    load_manifest_from_disk,
};
use crate::resolve::resolve_artifact_path;

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ValidationReport {
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
}

impl ValidationReport {
    pub fn is_valid(&self) -> bool {
        self.errors.is_empty()
    }

    fn push_error(&mut self, message: impl Into<String>) {
        self.errors.push(message.into());
    }

    fn push_warning(&mut self, message: impl Into<String>) {
        self.warnings.push(message.into());
    }
}

pub fn validate_manifest(manifest: &ProjectManifestFile) -> ValidationReport {
    let mut report = ValidationReport::default();

    if manifest.version != 1 {
        report.push_error(format!(
            "unsupported manifest version `{}` (expected 1)",
            manifest.version
        ));
    }

    let mut ids = std::collections::HashSet::new();
    for artifact in &manifest.artifacts {
        if !ids.insert(artifact.id.clone()) {
            report.push_error(format!("duplicate artifact id `{}`", artifact.id));
        }
    }

    for artifact in &manifest.artifacts {
        for dep in &artifact.depends_on {
            if !ids.contains(dep) {
                report.push_error(format!(
                    "artifact `{}` depends on unknown id `{}`",
                    artifact.id, dep
                ));
            }
        }
        for blocked in &artifact.blocks {
            if !ids.contains(blocked) {
                report.push_error(format!(
                    "artifact `{}` blocks unknown id `{}`",
                    artifact.id, blocked
                ));
            }
        }
        for artifact_ref in &artifact.refs {
            if !ids.contains(&artifact_ref.id) {
                report.push_warning(format!(
                    "artifact `{}` ref `{}` is not listed as a top-level artifact",
                    artifact.id, artifact_ref.id
                ));
            }
        }
    }

    if let Some(suggested_next) = manifest.build_track.suggested_next.as_deref() {
        match manifest.artifact(suggested_next) {
            None => report.push_error(format!(
                "build_track.suggested_next references unknown id `{suggested_next}`"
            )),
            Some(artifact) if artifact.kind != ArtifactKind::BuildPhase => report.push_error(
                format!("build_track.suggested_next `{suggested_next}` is not a build_phase"),
            ),
            _ => {}
        }
    }

    report
}

pub fn validate_manifest_on_disk(
    worktree_root: &Path,
    manifest: &ProjectManifestFile,
) -> ValidationReport {
    let mut report = validate_manifest(manifest);

    for artifact in &manifest.artifacts {
        let absolute = resolve_artifact_path(worktree_root, &manifest.roots, artifact);
        if !absolute.is_file() {
            report.push_error(format!(
                "artifact `{}` path does not exist: {}",
                artifact.id,
                absolute.display()
            ));
        }
    }

    report
}

pub fn validate_worktree(worktree_root: &Path) -> Result<ValidationReport> {
    let Some(loaded) = load_manifest_from_disk(worktree_root)? else {
        bail!("no plan manifest found under {}", worktree_root.display());
    };
    Ok(validate_manifest_on_disk(worktree_root, &loaded.manifest))
}

pub fn validate_worktree_or_error(worktree_root: &Path) -> Result<()> {
    let report = validate_worktree(worktree_root)?;
    if report.is_valid() {
        for warning in report.warnings {
            log::warn!("plan manifest warning: {warning}");
        }
        Ok(())
    } else {
        Err(anyhow::anyhow!(report.errors.join("\n")))
    }
}

pub fn ensure_manifest_exists(worktree_root: &Path) -> Result<LoadedManifest> {
    load_manifest_from_disk(worktree_root)?
        .with_context(|| format!("no plan manifest under {}", worktree_root.display()))
}

pub fn manifest_path_for_worktree(worktree_root: &Path) -> Result<std::path::PathBuf> {
    find_manifest_path(worktree_root)
        .with_context(|| format!("no plan manifest under {}", worktree_root.display()))
}

pub fn set_artifact_status(
    loaded: &mut LoadedManifest,
    artifact_id: &str,
    status: ArtifactStatus,
) -> Result<()> {
    let artifact = loaded
        .manifest
        .artifact_mut(artifact_id)
        .with_context(|| format!("unknown artifact id `{artifact_id}`"))?;
    ensure!(
        artifact.kind == ArtifactKind::BuildPhase,
        "artifact `{artifact_id}` is not a build phase"
    );
    artifact.status = status;
    crate::manifest::write_manifest(&loaded.manifest_path, &loaded.manifest)?;
    Ok(())
}
