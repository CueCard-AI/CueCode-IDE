use std::path::Path;

/// Fire when a workspace spec index finishes loading.
pub fn event_index_loaded(spec_count: usize, scan_duration_ms: u64) {
    telemetry::event!(
        "cuecode.spec.index_loaded",
        spec_count,
        scan_duration_ms
    );
}

/// Fire when the user attaches a spec via `@spec` in the composer.
pub fn event_mention_attached(spec_path: &Path, match_type: &str) {
    telemetry::event!(
        "cuecode.spec.mention_attached",
        spec_path = spec_path.display().to_string(),
        match_type
    );
}

/// Fire when a spec is linked to the active session (`link_spec` or `@spec`).
pub fn event_linked(spec_path: &Path) {
    telemetry::event!(
        "cuecode.spec.linked",
        spec_path = spec_path.display().to_string()
    );
}

/// Fire when the filesystem watcher refreshes the in-memory index.
pub fn event_watch_refresh(changed_files_count: usize) {
    telemetry::event!(
        "cuecode.spec.watch_refresh",
        changed_files_count
    );
}
