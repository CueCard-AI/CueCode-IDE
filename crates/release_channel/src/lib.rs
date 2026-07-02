//! Provides constructs for the Zed app version and release channel.

#![deny(missing_docs)]

use std::{str::FromStr, sync::LazyLock};

use env_var::read_env_var;
use gpui::{App, Global};
use semver::Version;

/// Base URL for CueCode documentation (stable channel).
pub const CUECODE_DOCS_URL: &str = "https://cuecode.dev/docs";

/// stable | dev | nightly | preview
pub static RELEASE_CHANNEL_NAME: LazyLock<String> = LazyLock::new(|| {
    if cfg!(debug_assertions) {
        read_env_var("CUECODE_RELEASE_CHANNEL", "ZED_RELEASE_CHANNEL")
            .value
            .unwrap_or_else(|| include_str!("../../cuecode/RELEASE_CHANNEL").trim().to_string())
    } else {
        include_str!("../../cuecode/RELEASE_CHANNEL").trim().to_string()
    }
});

#[doc(hidden)]
pub static RELEASE_CHANNEL: LazyLock<ReleaseChannel> =
    LazyLock::new(|| match ReleaseChannel::from_str(&RELEASE_CHANNEL_NAME) {
        Ok(channel) => channel,
        _ => panic!("invalid release channel {}", *RELEASE_CHANNEL_NAME),
    });

/// The app identifier for the current release channel, Windows only.
#[cfg(target_os = "windows")]
pub fn app_identifier() -> &'static str {
    match *RELEASE_CHANNEL {
        ReleaseChannel::Dev => "CueCode-Editor-Dev",
        ReleaseChannel::Nightly => "CueCode-Editor-Nightly",
        ReleaseChannel::Preview => "CueCode-Editor-Preview",
        ReleaseChannel::Stable => "CueCode-Editor-Stable",
    }
}

/// The Git commit SHA that Zed was built at.
#[derive(Clone, Eq, Debug, PartialEq)]
pub struct AppCommitSha(String);

struct GlobalAppCommitSha(AppCommitSha);

impl Global for GlobalAppCommitSha {}

impl AppCommitSha {
    /// Creates a new [`AppCommitSha`].
    pub fn new(sha: String) -> Self {
        AppCommitSha(sha)
    }

    /// Returns the global [`AppCommitSha`], if one is set.
    pub fn try_global(cx: &App) -> Option<AppCommitSha> {
        cx.try_global::<GlobalAppCommitSha>()
            .map(|sha| sha.0.clone())
    }

    /// Sets the global [`AppCommitSha`].
    pub fn set_global(sha: AppCommitSha, cx: &mut App) {
        cx.set_global(GlobalAppCommitSha(sha))
    }

    /// Returns the full commit SHA.
    pub fn full(&self) -> String {
        self.0.to_string()
    }

    /// Returns the short (7 character) commit SHA.
    pub fn short(&self) -> String {
        self.0.chars().take(7).collect()
    }
}

struct GlobalAppVersion(Version);

impl Global for GlobalAppVersion {}

/// The version of Zed.
pub struct AppVersion;

impl AppVersion {
    /// Load the app version from env.
    pub fn load(
        pkg_version: &str,
        build_id: Option<&str>,
        commit_sha: Option<AppCommitSha>,
    ) -> Version {
        let mut version: Version = if let Some(from_env) =
            read_env_var("CUECODE_APP_VERSION", "ZED_APP_VERSION").value
        {
            from_env.parse().expect("invalid CUECODE_APP_VERSION or ZED_APP_VERSION")
        } else {
            pkg_version.parse().expect("invalid version in Cargo.toml")
        };
        let mut pre = String::from(RELEASE_CHANNEL.dev_name());

        if let Some(build_id) = build_id {
            pre.push('.');
            pre.push_str(&build_id);
        }

        if let Some(sha) = commit_sha {
            pre.push('.');
            pre.push_str(&sha.0);
        }
        if let Ok(build) = semver::BuildMetadata::new(&pre) {
            version.build = build;
        }

        version
    }

    /// Returns the global version number.
    pub fn global(cx: &App) -> Version {
        if cx.has_global::<GlobalAppVersion>() {
            cx.global::<GlobalAppVersion>().0.clone()
        } else {
            Version::new(0, 0, 0)
        }
    }
}

/// A Zed release channel.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Default)]
pub enum ReleaseChannel {
    /// The development release channel.
    ///
    /// Used for local debug builds of Zed.
    #[default]
    Dev,

    /// The Nightly release channel.
    Nightly,

    /// The Preview release channel.
    Preview,

    /// The Stable release channel.
    Stable,
}

struct GlobalReleaseChannel(ReleaseChannel);

impl Global for GlobalReleaseChannel {}

/// Initializes the release channel.
pub fn init(app_version: Version, cx: &mut App) {
    cx.set_global(GlobalAppVersion(app_version));
    cx.set_global(GlobalReleaseChannel(*RELEASE_CHANNEL))
}

/// Initializes the release channel for tests that rely on fake release channel.
pub fn init_test(app_version: Version, release_channel: ReleaseChannel, cx: &mut App) {
    cx.set_global(GlobalAppVersion(app_version));
    cx.set_global(GlobalReleaseChannel(release_channel))
}

/// Returns the Zed docs URL for the current release channel for the given
/// `slug`.
pub fn docs_url(slug: &str, cx: &App) -> String {
    ReleaseChannel::try_global(cx)
        .unwrap_or(*RELEASE_CHANNEL)
        .docs_url(slug)
}

impl ReleaseChannel {
    /// All release channels.
    pub const ALL: [ReleaseChannel; 4] = [
        ReleaseChannel::Dev,
        ReleaseChannel::Nightly,
        ReleaseChannel::Preview,
        ReleaseChannel::Stable,
    ];

    /// Returns the global [`ReleaseChannel`].
    pub fn global(cx: &App) -> Self {
        cx.global::<GlobalReleaseChannel>().0
    }

    /// Returns the global [`ReleaseChannel`], if one is set.
    pub fn try_global(cx: &App) -> Option<Self> {
        cx.try_global::<GlobalReleaseChannel>()
            .map(|channel| channel.0)
    }

    /// Returns whether we want to poll for updates for this [`ReleaseChannel`]
    pub fn poll_for_updates(&self) -> bool {
        // CueCode v1: no auto-update polling to zed.dev; releases via cuecode.dev later.
        false
    }

    /// Returns the display name for this [`ReleaseChannel`].
    pub fn display_name(&self) -> &'static str {
        match self {
            ReleaseChannel::Dev => "CueCode Dev",
            ReleaseChannel::Nightly => "CueCode Nightly",
            ReleaseChannel::Preview => "CueCode Preview",
            ReleaseChannel::Stable => "CueCode",
        }
    }

    /// Returns the programmatic name for this [`ReleaseChannel`].
    pub fn dev_name(&self) -> &'static str {
        match self {
            ReleaseChannel::Dev => "dev",
            ReleaseChannel::Nightly => "nightly",
            ReleaseChannel::Preview => "preview",
            ReleaseChannel::Stable => "stable",
        }
    }

    /// Returns the application ID that's used by Wayland as application ID
    /// and WM_CLASS on X11.
    /// This also has to match the bundle identifier for Zed on macOS.
    pub fn app_id(&self) -> &'static str {
        match self {
            ReleaseChannel::Dev => "dev.cuecode.CueCode-Dev",
            ReleaseChannel::Nightly => "dev.cuecode.CueCode-Nightly",
            ReleaseChannel::Preview => "dev.cuecode.CueCode-Preview",
            ReleaseChannel::Stable => "dev.cuecode.CueCode",
        }
    }

    /// Returns the query parameter for this [`ReleaseChannel`].
    pub fn release_query_param(&self) -> Option<&'static str> {
        match self {
            Self::Dev => None,
            Self::Nightly => Some("nightly=1"),
            Self::Preview => Some("preview=1"),
            Self::Stable => None,
        }
    }

    /// Returns the Zed docs URL for this [`ReleaseChannel`] for the given
    /// `slug`.
    pub fn docs_url(&self, slug: &str) -> String {
        let channel_path_segment = match self {
            Self::Dev | Self::Nightly => Some("nightly"),
            Self::Preview => Some("preview"),
            Self::Stable => None,
        };

        match channel_path_segment {
            Some(channel) if slug.is_empty() => format!("{CUECODE_DOCS_URL}/{channel}"),
            Some(channel) => format!("{CUECODE_DOCS_URL}/{channel}/{slug}"),
            None if slug.is_empty() => CUECODE_DOCS_URL.to_string(),
            None => format!("{CUECODE_DOCS_URL}/{slug}"),
        }
    }
}

/// Error indicating that release channel string does not match any known release channel names.
#[derive(Copy, Clone, Debug, Hash, PartialEq)]
pub struct InvalidReleaseChannel;

impl FromStr for ReleaseChannel {
    type Err = InvalidReleaseChannel;

    fn from_str(channel: &str) -> Result<Self, Self::Err> {
        Ok(match channel {
            "dev" => ReleaseChannel::Dev,
            "nightly" => ReleaseChannel::Nightly,
            "preview" => ReleaseChannel::Preview,
            "stable" => ReleaseChannel::Stable,
            _ => return Err(InvalidReleaseChannel),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::ReleaseChannel;

    #[test]
    fn test_docs_url_for_release_channel() {
        assert_eq!(
            ReleaseChannel::Dev.docs_url("settings"),
            "https://cuecode.dev/docs/nightly/settings"
        );
        assert_eq!(
            ReleaseChannel::Nightly.docs_url("settings"),
            "https://cuecode.dev/docs/nightly/settings"
        );
        assert_eq!(
            ReleaseChannel::Preview.docs_url("settings"),
            "https://cuecode.dev/docs/preview/settings"
        );
        assert_eq!(
            ReleaseChannel::Stable.docs_url("settings"),
            "https://cuecode.dev/docs/settings"
        );
    }
}
