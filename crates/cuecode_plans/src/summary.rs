pub const PLAN_DIGEST_PROMPT_BUDGET: usize = 2048;

use crate::manifest::{ArtifactStatus, ProjectManifestFile};

pub fn format_plan_digest_markdown(manifest: &ProjectManifestFile) -> String {
    let mut out = String::from(
        "CueCode build track (from `.cuecode/plans/project.yaml`):\n\n\
         | Phase | Status | Deps ready |\n| --- | --- | --- |\n",
    );

    for artifact in manifest.build_phases() {
        let deps_ready = if manifest.deps_satisfied(artifact) {
            "yes"
        } else {
            "no"
        };
        let line = format!(
            "| {} | {} | {} |\n",
            artifact.id,
            status_label(artifact.status),
            deps_ready
        );
        if out.len() + line.len() > PLAN_DIGEST_PROMPT_BUDGET {
            out.push_str("\n*(plan digest truncated)*\n");
            break;
        }
        out.push_str(&line);
    }

    if let Some(suggested) = manifest.effective_suggested_next() {
        let line = format!("\nSuggested next phase: `{suggested}`\n");
        if out.len() + line.len() <= PLAN_DIGEST_PROMPT_BUDGET {
            out.push_str(&line);
        }
    }

    out
}

pub fn status_label(status: ArtifactStatus) -> &'static str {
    match status {
        ArtifactStatus::NotStarted => "not_started",
        ArtifactStatus::InProgress => "in_progress",
        ArtifactStatus::Done => "done",
    }
}

pub fn build_track_status_label(status: ArtifactStatus) -> &'static str {
    match status {
        ArtifactStatus::NotStarted => "Not started",
        ArtifactStatus::InProgress => "In progress",
        ArtifactStatus::Done => "Done",
    }
}
