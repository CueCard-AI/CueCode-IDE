use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

pub const MANIFEST_CANONICAL_REL: &str = ".cuecode/plans/project.yaml";
pub const MANIFEST_ALIAS_REL: &str = ".cuecode/project.yaml";

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProjectManifestFile {
    pub version: u32,
    pub project: ProjectInfo,
    #[serde(default)]
    pub roots: ManifestRoots,
    pub artifacts: Vec<Artifact>,
    #[serde(default)]
    pub build_track: BuildTrack,
    #[serde(default)]
    pub docs: ManifestDocs,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProjectInfo {
    pub name: String,
    pub adopted_at: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct ManifestRoots {
    #[serde(default)]
    pub plans: Option<String>,
    #[serde(default)]
    pub spec: Option<String>,
    #[serde(default)]
    pub cursor_mirror: Option<String>,
    #[serde(default)]
    pub merge_cursor: bool,
    #[serde(default)]
    pub mirror_policy: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BuildTrack {
    #[serde(default)]
    pub suggested_next: Option<String>,
}

impl Default for BuildTrack {
    fn default() -> Self {
        Self {
            suggested_next: None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct ManifestDocs {
    #[serde(default)]
    pub readme: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Artifact {
    pub id: String,
    pub kind: ArtifactKind,
    pub path: String,
    pub root: ArtifactRoot,
    #[serde(default)]
    pub canonical: bool,
    #[serde(default)]
    pub status: ArtifactStatus,
    #[serde(default)]
    pub depends_on: Vec<String>,
    #[serde(default)]
    pub blocks: Vec<String>,
    #[serde(default)]
    pub pin_policy: Option<PinPolicy>,
    #[serde(default)]
    pub refs: Vec<ArtifactRef>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ArtifactKind {
    BuildPhase,
    ProductPlan,
    FeatureSpec,
    Readme,
    Ignore,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ArtifactRoot {
    Plans,
    Spec,
    External,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum ArtifactStatus {
    #[default]
    NotStarted,
    InProgress,
    Done,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum PinPolicy {
    #[default]
    Summary,
    Off,
    Full,
    Section,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ArtifactRef {
    pub id: String,
    pub path: String,
    pub root: ArtifactRoot,
    #[serde(default)]
    pub role: RefRole,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum RefRole {
    #[default]
    Context,
    Required,
    Optional,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LoadedManifest {
    pub manifest: ProjectManifestFile,
    pub manifest_path: PathBuf,
}

impl ProjectManifestFile {
    pub fn artifact(&self, id: &str) -> Option<&Artifact> {
        self.artifacts.iter().find(|artifact| artifact.id == id)
    }

    pub fn artifact_mut(&mut self, id: &str) -> Option<&mut Artifact> {
        self.artifacts.iter_mut().find(|artifact| artifact.id == id)
    }

    pub fn build_phases(&self) -> impl Iterator<Item = &Artifact> {
        self.artifacts
            .iter()
            .filter(|artifact| artifact.kind == ArtifactKind::BuildPhase)
    }

    pub fn deps_satisfied(&self, artifact: &Artifact) -> bool {
        artifact.depends_on.iter().all(|dep| {
            self.artifact(dep)
                .is_some_and(|dep_artifact| dep_artifact.status == ArtifactStatus::Done)
        })
    }

    pub fn effective_suggested_next(&self) -> Option<&str> {
        if let Some(id) = self.build_track.suggested_next.as_deref() {
            if self
                .artifact(id)
                .is_some_and(|artifact| artifact.kind == ArtifactKind::BuildPhase)
            {
                return Some(id);
            }
        }

        self.build_phases()
            .filter(|artifact| artifact.status != ArtifactStatus::Done && self.deps_satisfied(artifact))
            .map(|artifact| artifact.id.as_str())
            .next()
    }
}

pub fn manifest_paths(worktree_root: &Path) -> [PathBuf; 2] {
    [
        worktree_root.join(MANIFEST_CANONICAL_REL),
        worktree_root.join(MANIFEST_ALIAS_REL),
    ]
}

pub fn find_manifest_path(worktree_root: &Path) -> Option<PathBuf> {
    manifest_paths(worktree_root)
        .into_iter()
        .find(|path| path.is_file())
}

pub fn load_manifest_from_disk(worktree_root: &Path) -> anyhow::Result<Option<LoadedManifest>> {
    let Some(manifest_path) = find_manifest_path(worktree_root) else {
        return Ok(None);
    };
    let source = std::fs::read_to_string(&manifest_path)?;
    let manifest: ProjectManifestFile = serde_yaml::from_str(&source)?;
    Ok(Some(LoadedManifest {
        manifest,
        manifest_path,
    }))
}

pub fn write_manifest(path: &Path, manifest: &ProjectManifestFile) -> anyhow::Result<()> {
    let yaml = serde_yaml::to_string(manifest)?;
    std::fs::write(path, yaml)?;
    Ok(())
}
