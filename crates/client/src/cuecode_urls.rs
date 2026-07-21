//! Contains helper functions for constructing URLs to CueCode-related pages.
//!
//! CueCode v1: cloud billing/account URLs are disabled. Docs links use the
//! release channel docs path when a server URL is configured.

use gpui::App;
use release_channel::ReleaseChannel;
use release_channel::CUECODE_DOCS_URL;
use settings::Settings;

use crate::ClientSettings;

fn server_url(cx: &App) -> &str {
    &ClientSettings::get_global(cx).server_url
}

fn docs_url(cx: &App) -> String {
    let server_url = server_url(cx);
    if server_url.is_empty() {
        return ReleaseChannel::try_global(cx)
            .unwrap_or_default()
            .docs_url("");
    }
    match ReleaseChannel::try_global(cx).unwrap_or_default() {
        ReleaseChannel::Stable => {
            format!("{server_url}/docs")
        }
        ReleaseChannel::Preview => {
            format!("{server_url}/docs/preview")
        }
        ReleaseChannel::Dev | ReleaseChannel::Nightly => {
            format!("{server_url}/docs/nightly")
        }
    }
}

/// Returns the URL to the account page. Disabled in CueCode v1.
pub fn account_url(_cx: &App) -> String {
    String::new()
}

/// Returns the URL to the start trial page. Disabled in CueCode v1.
pub fn start_trial_url(_cx: &App) -> String {
    String::new()
}

/// Returns the URL to the upgrade page. Disabled in CueCode v1.
pub fn upgrade_to_pro_url(_cx: &App) -> String {
    String::new()
}

/// Deprecated alias for [`upgrade_to_pro_url`].
pub use upgrade_to_pro_url as upgrade_to_zed_pro_url;

/// Returns the URL to the terms of service.
pub fn terms_of_service(cx: &App) -> String {
    let server_url = server_url(cx);
    if server_url.is_empty() {
        return String::new();
    }
    format!("{server_url}/terms-of-service")
}

pub fn releases_url() -> &'static str {
    "https://cuecode.dev/releases"
}

/// Returns a stable-channel docs URL for the given slug (empty for the docs home page).
pub fn stable_docs_url(slug: &str) -> String {
    if slug.is_empty() {
        format!("{CUECODE_DOCS_URL}/")
    } else {
        format!("{CUECODE_DOCS_URL}/{slug}")
    }
}

/// Returns the URL to AI privacy and security docs.
pub fn ai_privacy_and_security(cx: &App) -> String {
    format!(
        "{docs_url}/ai/privacy-and-security",
        docs_url = docs_url(cx)
    )
}

/// Returns the URL to edit prediction documentation.
pub fn edit_prediction_docs(cx: &App) -> String {
    format!("{docs_url}/ai/edit-prediction", docs_url = docs_url(cx))
}

pub fn skills_docs(cx: &App) -> String {
    format!("{docs_url}/ai/skills", docs_url = docs_url(cx))
}

/// Returns the URL to the ACP registry blog post.
pub fn acp_registry_blog(cx: &App) -> String {
    let server_url = server_url(cx);
    if server_url.is_empty() {
        return String::new();
    }
    format!("{server_url}/blog/acp-registry")
}

pub fn shared_agent_thread_url(session_id: &str) -> String {
    format!("cuecode://agent/shared/{}", session_id)
}
