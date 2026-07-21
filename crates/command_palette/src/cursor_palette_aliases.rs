//! Extra command-palette search strings for users coming from Cursor / VS Code.
//!
//! These aliases are fuzzy-matched alongside the humanized action name so queries
//! like "developer reload" surface `cuecode::ReloadWindow`.

/// Returns additional palette search strings for the given registered action name.
pub fn aliases_for_action(action_name: &str) -> &'static [&'static str] {
    match action_name {
        "cuecode::ReloadWindow" => &[
            "Developer: Reload Window",
            "Reload Window",
            "developer reload",
            "reload window",
        ],
        "agent::OpenSettings" => &[
            "Cursor: Open Settings",
            "Agent Settings",
            "Cursor Settings",
            "Preferences: Open Agent Settings",
            "Open Agent Settings",
        ],
        "cuecode::OpenSettings" => &[
            "Preferences: Open Settings",
            "Open Settings",
            "Cursor Settings",
        ],
        "cuecode::OpenSettingsFile" => &[
            "Preferences: Open User Settings (JSON)",
            "Open Settings JSON",
            "Open User Settings",
        ],
        "command_palette::Toggle" => &[
            "Show All Commands",
            "Command Palette",
            "Cursor: Command Palette",
        ],
        "agent::ToggleFocus" => &[
            "Open Chat",
            "Toggle Chat",
            "Cursor Chat",
            "Focus Chat",
        ],
        "assistant::InlineAssist" => &[
            "Inline Edit",
            "Cursor Inline Edit",
            "Inline Chat",
        ],
        "cuecode::OpenKeymap" => &[
            "Open Keyboard Shortcuts",
            "Preferences: Open Keyboard Shortcuts",
            "Keyboard Shortcuts",
        ],
        "agent::NewThread" => &["New Chat", "New Agent Chat", "Cursor: New Chat"],
        "cuecode::Quit" => &["Quit", "Developer: Quit", "Exit"],
        "workspace::ToggleRightDock" => &["Toggle Chat Panel", "Toggle Agent Panel"],
        "agent::ToggleModelSelector" => &["Change Model", "Select Model"],
        "agent::AddSelectionToThread" => &[
            "Add to Chat",
            "Add Selection to Chat",
            "Ask Cursor",
        ],
        _ => &[],
    }
}
