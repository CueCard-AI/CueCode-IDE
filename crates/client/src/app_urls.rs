//! CueCode deep-link URL scheme (`cuecode://`) with legacy `zed://` compatibility.

use std::borrow::Cow;

/// Canonical application URL scheme.
pub const CUECODE_URL_SCHEME: &str = "cuecode";

/// Legacy URL scheme retained for backward compatibility with Zed links.
pub const ZED_URL_SCHEME: &str = "zed";

const CUECODE_PREFIX: &str = "cuecode://";
const ZED_PREFIX: &str = "zed://";

/// URL schemes registered with the OS for deep links.
pub const APP_URL_SCHEMES: [&str; 1] = [CUECODE_URL_SCHEME];

/// Returns the canonical `cuecode://` prefix.
pub fn cuecode_url_prefix() -> &'static str {
    CUECODE_PREFIX
}

/// Returns the legacy `zed://` prefix.
pub fn legacy_zed_url_prefix() -> &'static str {
    ZED_PREFIX
}

/// Returns true when `url` uses `cuecode://` or legacy `zed://`.
pub fn is_app_deep_link(url: &str) -> bool {
    url.starts_with(CUECODE_PREFIX) || url.starts_with(ZED_PREFIX)
}

/// Normalize incoming deep links to the canonical `cuecode://` form.
pub fn normalize_incoming_url(url: &str) -> Cow<'_, str> {
    if let Some(rest) = url.strip_prefix(ZED_PREFIX) {
        Cow::Owned(format!("{CUECODE_PREFIX}{rest}"))
    } else {
        Cow::Borrowed(url)
    }
}

/// Returns the portion of a deep link after `cuecode://` or legacy `zed://`.
pub fn strip_app_url_prefix(url: &str) -> Option<&str> {
    url.strip_prefix(CUECODE_PREFIX)
        .or_else(|| url.strip_prefix(ZED_PREFIX))
}
