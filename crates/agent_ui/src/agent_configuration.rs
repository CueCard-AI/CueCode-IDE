pub mod configure_context_server_modal;
mod manage_profiles_modal;
mod tool_picker;

use std::sync::Arc;

use context_server::ContextServerId;
use extension::ExtensionManifest;
use extension_host::ExtensionStore;
use gpui::App;

pub(crate) use configure_context_server_modal::ConfigureContextServerModal;
pub(crate) use manage_profiles_modal::ManageProfilesModal;

pub(crate) fn resolve_extension_for_context_server(
    id: &ContextServerId,
    cx: &App,
) -> Option<(Arc<str>, Arc<ExtensionManifest>)> {
    ExtensionStore::global(cx)
        .read(cx)
        .installed_extensions()
        .iter()
        .find(|(_, entry)| entry.manifest.context_servers.contains_key(&id.0))
        .map(|(id, entry)| (id.clone(), entry.manifest.clone()))
}

#[cfg(test)]
fn remove_compatible_provider(settings: &mut SettingsContent, provider_id: &str) {
    // Mirrors the OpenAI-wins precedence used at registration time: only the
    // entry that is actually registered gets removed. A shadowed
    // `anthropic_compatible` entry with the same name takes over instead of
    // being silently deleted.
    let Some(language_models) = settings.language_models.as_mut() else {
        return;
    };
    let removed_from_openai = language_models
        .openai_compatible
        .as_mut()
        .and_then(|providers| providers.remove(provider_id))
        .is_some();
    if !removed_from_openai && let Some(providers) = language_models.anthropic_compatible.as_mut() {
        providers.remove(provider_id);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use collections::HashMap;
    use settings::{AnthropicCompatibleSettingsContent, OpenAiCompatibleSettingsContent, SettingsContent};

    fn settings_with_compatible_providers(openai: &[&str], anthropic: &[&str]) -> SettingsContent {
        let mut settings = SettingsContent::default();
        let language_models = settings.language_models.get_or_insert_default();
        language_models.openai_compatible = Some(
            openai
                .iter()
                .map(|id| {
                    (
                        Arc::from(*id),
                        OpenAiCompatibleSettingsContent {
                            api_url: "https://example.com".to_string(),
                            available_models: Vec::new(),
                            custom_headers: None,
                        },
                    )
                })
                .collect(),
        );
        language_models.anthropic_compatible = Some(
            anthropic
                .iter()
                .map(|id| {
                    (
                        Arc::from(*id),
                        AnthropicCompatibleSettingsContent {
                            api_url: "https://example.com".to_string(),
                            available_models: Vec::new(),
                            custom_headers: None,
                        },
                    )
                })
                .collect(),
        );
        settings
    }

    fn compatible_provider_keys(settings: &SettingsContent) -> (Vec<&str>, Vec<&str>) {
        fn keys<T>(providers: Option<&HashMap<Arc<str>, T>>) -> Vec<&str> {
            providers
                .map(|providers| providers.keys().map(|key| key.as_ref()).collect())
                .unwrap_or_default()
        }

        let language_models = settings
            .language_models
            .as_ref()
            .expect("language_models settings should exist");
        (
            keys(language_models.openai_compatible.as_ref()),
            keys(language_models.anthropic_compatible.as_ref()),
        )
    }

    #[test]
    fn test_remove_compatible_provider_openai_only() {
        let mut settings = settings_with_compatible_providers(&["acme"], &[]);
        remove_compatible_provider(&mut settings, "acme");
        let (openai, anthropic) = compatible_provider_keys(&settings);
        assert_eq!(openai, Vec::<&str>::new());
        assert_eq!(anthropic, Vec::<&str>::new());
    }

    #[test]
    fn test_remove_compatible_provider_anthropic_only() {
        let mut settings = settings_with_compatible_providers(&[], &["acme"]);
        remove_compatible_provider(&mut settings, "acme");
        let (openai, anthropic) = compatible_provider_keys(&settings);
        assert_eq!(openai, Vec::<&str>::new());
        assert_eq!(anthropic, Vec::<&str>::new());
    }

    #[test]
    fn test_remove_compatible_provider_collision_removes_only_openai_entry() {
        let mut settings = settings_with_compatible_providers(&["acme"], &["acme"]);

        remove_compatible_provider(&mut settings, "acme");
        let (openai, anthropic) = compatible_provider_keys(&settings);
        assert_eq!(
            openai,
            Vec::<&str>::new(),
            "the registered (OpenAI-compatible) entry should be removed"
        );
        assert_eq!(
            anthropic,
            vec!["acme"],
            "the shadowed anthropic_compatible entry should survive"
        );

        // A second removal deletes the entry that took over.
        remove_compatible_provider(&mut settings, "acme");
        let (_, anthropic) = compatible_provider_keys(&settings);
        assert_eq!(anthropic, Vec::<&str>::new());
    }

    #[test]
    fn test_remove_compatible_provider_leaves_other_providers_untouched() {
        let mut settings = settings_with_compatible_providers(&["acme", "globex"], &["initech"]);
        remove_compatible_provider(&mut settings, "acme");
        let (openai, anthropic) = compatible_provider_keys(&settings);
        assert_eq!(openai, vec!["globex"]);
        assert_eq!(anthropic, vec!["initech"]);
    }
}
