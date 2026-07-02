use std::path::PathBuf;
use std::sync::Arc;

use agent_client_protocol::schema as acp;
use anyhow::{Context as _, Result};
use cuecode_plans::{
    ArtifactStatus, ImplementBundle, build_implement_bundle, count_checkboxes_in_file,
    load_manifest_from_disk, register_plan_manifest, resolve_artifact_path, set_artifact_status,
};
use agent_settings::AgentProfileId;
use fs::Fs;
use gpui::{App, Context, Entity, SharedString, Window};
use settings::WorktreeId;
use workspace::Workspace;

use crate::agent_panel::CreateThreadOptions;
use crate::{
    AgentInitialContent, AgentPanel, AgentThreadSource, ConversationView,
};

pub fn implement_build_phase(
    workspace: &mut Workspace,
    worktree_id: WorktreeId,
    worktree_root: PathBuf,
    artifact_id: SharedString,
    window: &mut Window,
    cx: &mut Context<Workspace>,
) -> Result<()> {
    let mut loaded = load_manifest_from_disk(&worktree_root)
        .with_context(|| format!("loading plan manifest for {}", worktree_root.display()))?
        .with_context(|| "no plan manifest found")?;

    set_artifact_status(&mut loaded, &artifact_id, ArtifactStatus::InProgress)?;
    register_plan_manifest(
        <dyn Fs>::global(cx),
        worktree_root.clone(),
        worktree_id,
        Arc::new(loaded),
        cx,
    );

    let loaded = load_manifest_from_disk(&worktree_root)
        .with_context(|| "reloading plan manifest after status update")?
        .with_context(|| "no plan manifest found")?;
    let bundle = build_implement_bundle(&worktree_root, &loaded.manifest, &artifact_id)?;
    let task_progress = loaded
        .manifest
        .artifact(artifact_id.as_ref())
        .map(|artifact| {
            let absolute = resolve_artifact_path(&worktree_root, &loaded.manifest.roots, artifact);
            count_checkboxes_in_file(&absolute)
                .unwrap_or_default()
                .label()
        })
        .unwrap_or_else(|| "0/0".to_string());
    let ref_count = bundle.refs.len();

    let Some(panel) = workspace.panel::<AgentPanel>(cx) else {
        anyhow::bail!("agent panel unavailable");
    };

    panel.update(cx, |panel, cx| {
        start_ticket_session(panel, &bundle, &artifact_id, window, cx);
        panel.collapse_plan_after_implement(
            artifact_id.as_ref(),
            task_progress,
            ref_count,
            window,
            cx,
        );
    });
    workspace.focus_panel::<AgentPanel>(window, cx);

    let workspace_entity = cx.entity();
    crate::layout_studio::prompt_layout_studio_for_implement(&workspace_entity, window, cx);

    Ok(())
}

pub fn mark_build_phase_done(
    worktree_root: &PathBuf,
    worktree_id: WorktreeId,
    artifact_id: &str,
    cx: &mut App,
) -> Result<()> {
    let mut loaded = load_manifest_from_disk(worktree_root)
        .with_context(|| format!("loading plan manifest for {}", worktree_root.display()))?
        .with_context(|| "no plan manifest found")?;
    set_artifact_status(&mut loaded, artifact_id, ArtifactStatus::Done)?;
    register_plan_manifest(
        <dyn Fs>::global(cx),
        worktree_root.clone(),
        worktree_id,
        Arc::new(loaded),
        cx,
    );
    Ok(())
}

fn start_ticket_session(
    panel: &mut AgentPanel,
    bundle: &ImplementBundle,
    artifact_id: &SharedString,
    window: &mut Window,
    cx: &mut Context<AgentPanel>,
) {
    let title = SharedString::from(format!("Implement {artifact_id}"));
    let stub = bundle.composer_stub.clone();
    let primary_path = bundle.primary_path.clone();
    let ticket_id = bundle.ticket_id.clone();

    let thread_id = panel.create_thread_with_options(
        CreateThreadOptions {
            title: Some(title),
            initial_content: Some(AgentInitialContent::ContentBlock {
                blocks: vec![acp::ContentBlock::Text(acp::TextContent::new(stub))],
                auto_submit: false,
            }),
            ..Default::default()
        },
        AgentThreadSource::AgentPanel,
        window,
        cx,
    );

    panel.activate_retained_thread(thread_id, true, window, cx);

    if let Some(conversation_view) = panel.active_conversation_view() {
        configure_ticket_session(conversation_view.clone(), primary_path, ticket_id, cx);
    }
}

fn configure_ticket_session(
    conversation_view: Entity<ConversationView>,
    primary_path: PathBuf,
    ticket_id: String,
    cx: &mut Context<AgentPanel>,
) {
    conversation_view.update(cx, |conversation_view, cx| {
        if let Some(native_thread) = conversation_view.as_native_thread(cx) {
            native_thread.update(cx, |thread, cx| {
                thread.set_active_spec_path(Some(primary_path));
                thread.set_active_ticket_id(Some(ticket_id));
                thread.set_profile(AgentProfileId("write".into()), cx);
            });
        } else if let Some(acp_thread) = conversation_view.root_thread(cx) {
            acp_thread.update(cx, |thread, cx| {
                thread.set_active_spec_path(Some(primary_path), cx);
                thread.set_active_ticket_id(Some(ticket_id), cx);
            });
        }
    });
}
