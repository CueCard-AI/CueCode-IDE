use std::path::{Path, PathBuf};

use crate::manifest::{Artifact, ArtifactRef, ArtifactRoot, ManifestRoots, ProjectManifestFile};

pub fn resolve_artifact_path(
    worktree_root: &Path,
    roots: &ManifestRoots,
    artifact: &Artifact,
) -> PathBuf {
    resolve_rooted_path(worktree_root, roots, artifact.root, &artifact.path)
}

pub fn resolve_ref_path(
    worktree_root: &Path,
    roots: &ManifestRoots,
    artifact_ref: &ArtifactRef,
) -> PathBuf {
    resolve_rooted_path(
        worktree_root,
        roots,
        artifact_ref.root,
        &artifact_ref.path,
    )
}

fn resolve_rooted_path(
    worktree_root: &Path,
    roots: &ManifestRoots,
    root: ArtifactRoot,
    path: &str,
) -> PathBuf {
    match root {
        ArtifactRoot::Plans => {
            let plans_root = roots
                .plans
                .as_deref()
                .unwrap_or(".cuecode/plans");
            worktree_root.join(plans_root).join(path)
        }
        ArtifactRoot::Spec => {
            let spec_root = roots.spec.as_deref().unwrap_or(".cuecode/specs");
            worktree_root.join(spec_root).join(path)
        }
        ArtifactRoot::External => worktree_root.join(path),
    }
}

/// Project-relative path suitable for `active_spec_path` (uses forward slashes).
pub fn project_relative_path(worktree_root: &Path, absolute_path: &Path) -> Option<PathBuf> {
    absolute_path
        .strip_prefix(worktree_root)
        .ok()
        .map(|path| path.to_path_buf())
}

pub fn resolve_project_relative_path(
    worktree_root: &Path,
    roots: &ManifestRoots,
    artifact: &Artifact,
) -> PathBuf {
    let absolute = resolve_artifact_path(worktree_root, roots, artifact);
    project_relative_path(worktree_root, &absolute).unwrap_or_else(|| artifact.path.clone().into())
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResolvedRef {
    pub id: String,
    pub path: PathBuf,
    pub role: crate::manifest::RefRole,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ImplementBundle {
    pub ticket_id: String,
    pub title: String,
    pub primary_path: PathBuf,
    pub refs: Vec<ResolvedRef>,
    pub composer_stub: String,
}

pub fn build_implement_bundle(
    worktree_root: &Path,
    manifest: &ProjectManifestFile,
    ticket_id: &str,
) -> anyhow::Result<ImplementBundle> {
    let artifact = manifest
        .artifact(ticket_id)
        .ok_or_else(|| anyhow::anyhow!("unknown artifact id `{ticket_id}`"))?;
    anyhow::ensure!(
        artifact.kind == crate::manifest::ArtifactKind::BuildPhase,
        "artifact `{ticket_id}` is not a build phase"
    );

    let primary_path = resolve_project_relative_path(worktree_root, &manifest.roots, artifact);
    let refs = artifact
        .refs
        .iter()
        .map(|artifact_ref| ResolvedRef {
            id: artifact_ref.id.clone(),
            path: resolve_project_relative_path_from_ref(worktree_root, &manifest.roots, artifact_ref),
            role: artifact_ref.role,
        })
        .collect::<Vec<_>>();

    let composer_stub = format_implement_stub(ticket_id, &primary_path, &refs);

    Ok(ImplementBundle {
        ticket_id: ticket_id.to_string(),
        title: artifact.id.clone(),
        primary_path,
        refs,
        composer_stub,
    })
}

fn resolve_project_relative_path_from_ref(
    worktree_root: &Path,
    roots: &ManifestRoots,
    artifact_ref: &ArtifactRef,
) -> PathBuf {
    let absolute = resolve_ref_path(worktree_root, roots, artifact_ref);
    project_relative_path(worktree_root, &absolute)
        .unwrap_or_else(|| artifact_ref.path.clone().into())
}

fn format_implement_stub(
    ticket_id: &str,
    primary_path: &Path,
    refs: &[ResolvedRef],
) -> String {
    let mut stub = format!(
        "Implement build phase `{ticket_id}`.\n\nPrimary phase doc: `{}`\n",
        primary_path.display()
    );

    let required = refs
        .iter()
        .filter(|artifact_ref| artifact_ref.role == crate::manifest::RefRole::Required)
        .collect::<Vec<_>>();
    if !required.is_empty() {
        stub.push_str("\nRequired refs (read early via `@spec` or tools):\n");
        for artifact_ref in required {
            stub.push_str(&format!(
                "- `{}` ({})\n",
                artifact_ref.path.display(),
                artifact_ref.id
            ));
        }
    }

    let context = refs
        .iter()
        .filter(|artifact_ref| artifact_ref.role == crate::manifest::RefRole::Context)
        .collect::<Vec<_>>();
    if !context.is_empty() {
        stub.push_str("\nContext refs:\n");
        for artifact_ref in context {
            stub.push_str(&format!(
                "- `{}` ({})\n",
                artifact_ref.path.display(),
                artifact_ref.id
            ));
        }
    }

    stub.push_str(
        "\nWork through the phase tasks in order. Run `./scripts/verify-all.sh` to confirm green before marking the phase done in the Planning Hub.",
    );
    stub
}
