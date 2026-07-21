use std::rc::Rc;

use anyhow::Result;
use gpui::{AsyncWindowContext, Context, Entity, Subscription, Task, Window};
use language_model::{ApiKeyState, AuthenticateError};
use ui::{Tooltip, prelude::*};
use ui_input::InputField;
use util::ResultExt;

/// The current credential state of a single-API-key provider, as reported by the
/// provider when constructing an [`ApiKeyEditor`].
pub enum ApiKeyStatus {
    /// No key is configured; show the input field.
    Unset,
    /// A key is configured via the UI; show a "configured" row with a reset.
    Configured,
    /// The key comes from an environment variable and can't be edited here.
    FromEnvVar(SharedString),
}

/// Maps a provider's [`ApiKeyState`] to the [`ApiKeyStatus`] the editor renders.
/// Shared so the API-key providers don't each duplicate this mapping.
pub fn api_key_status(state: &ApiKeyState) -> ApiKeyStatus {
    if state.is_from_env_var() {
        ApiKeyStatus::FromEnvVar(state.env_var_name().clone())
    } else if state.has_key() {
        ApiKeyStatus::Configured
    } else {
        ApiKeyStatus::Unset
    }
}

/// A compact, reusable control for editing a provider's single API key, intended
/// to be returned from `LanguageModelProvider::configuration_view_v2` as an
/// inline control.
///
/// It is deliberately provider-agnostic: the provider supplies closures that
/// read the current [`ApiKeyStatus`] and store/clear the key against its own
/// state, so all credential knowledge stays in the provider.
pub struct ApiKeyEditor {
    input: Entity<InputField>,
    api_key_url: SharedString,
    status: Rc<dyn Fn(&App) -> ApiKeyStatus>,
    set_key: Rc<dyn Fn(String, &mut AsyncWindowContext) -> Task<Result<()>>>,
    reset_key: Rc<dyn Fn(&mut AsyncWindowContext) -> Task<Result<()>>>,
    load_credentials_task: Option<Task<()>>,
    saving: bool,
    _subscription: Subscription,
}

impl ApiKeyEditor {
    pub fn new<S: 'static>(
        state: Entity<S>,
        api_key_url: impl Into<SharedString>,
        placeholder: &str,
        status: impl Fn(&S, &App) -> ApiKeyStatus + 'static,
        set_key: impl Fn(&Entity<S>, String, &mut AsyncWindowContext) -> Task<Result<()>> + 'static,
        reset_key: impl Fn(&Entity<S>, &mut AsyncWindowContext) -> Task<Result<()>> + 'static,
        authenticate: impl Fn(&mut S, &mut Context<S>) -> Task<Result<(), AuthenticateError>>
            + 'static,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let input = cx.new(|cx| {
            InputField::new(window, cx, placeholder)
                .masked(true)
                .tab_index(0)
        });
        let subscription = cx.observe(&state, |_, _, cx| cx.notify());

        let status_state = state.clone();
        let set_state = state.clone();
        let reset_state = state.clone();
        let load_state = state;

        let load_credentials_task = Some(cx.spawn_in(window, async move |this, cx| {
            let result = load_state.update(cx, authenticate).await;
            match result {
                Ok(()) | Err(AuthenticateError::CredentialsNotFound) => {}
                Err(error) => {
                    log::error!("Failed to load API key credentials: {error}");
                }
            }
            this.update(cx, |this, cx| {
                this.load_credentials_task = None;
                cx.notify();
            })
            .log_err();
        }));

        Self {
            input,
            api_key_url: api_key_url.into(),
            status: Rc::new(move |cx| status(status_state.read(cx), cx)),
            set_key: Rc::new(move |key, cx| set_key(&set_state, key, cx)),
            reset_key: Rc::new(move |cx| reset_key(&reset_state, cx)),
            load_credentials_task,
            saving: false,
            _subscription: subscription,
        }
    }

    fn save_key(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let key = self.input.read(cx).text(cx).trim().to_string();
        if key.is_empty() || self.saving {
            return;
        }

        self.input.update(cx, |input, cx| {
            input.set_error(None::<&str>, cx);
            input.set_text("", window, cx);
        });

        self.saving = true;
        cx.notify();

        let set_key = self.set_key.clone();
        cx.spawn_in(window, async move |this, cx| {
            let result = set_key(key, cx).await;
            this.update(cx, |this, cx| {
                this.saving = false;
                if let Err(error) = &result {
                    this.input.update(cx, |input, cx| {
                        input.set_error(Some(error.to_string()), cx);
                    });
                }
                cx.notify();
            })
            .log_err();
        })
        .detach();
    }

    fn save(&mut self, _: &menu::Confirm, window: &mut Window, cx: &mut Context<Self>) {
        self.save_key(window, cx);
    }

    fn reset(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let reset_key = self.reset_key.clone();
        cx.spawn_in(window, async move |_, cx| {
            reset_key(cx).await.log_err();
        })
        .detach();
    }

    fn render_where_to_find_key(&self) -> impl IntoElement {
        let url = self.api_key_url.clone();
        let click_url = url.to_string();
        h_flex()
            .id("where-to-find-key")
            .gap_0p5()
            .cursor_pointer()
            .child(
                Icon::new(IconName::Info)
                    .size(IconSize::XSmall)
                    .color(Color::Muted),
            )
            .child(
                Label::new("Where to find key")
                    .size(LabelSize::Small)
                    .color(Color::Muted),
            )
            .tooltip(Tooltip::text(format!("Create an API key at {url}")))
            .on_click(move |_, _window, cx| cx.open_url(&click_url))
    }
}

impl Render for ApiKeyEditor {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        if self.load_credentials_task.is_some() {
            return Label::new("Loading credentials…")
                .size(LabelSize::Small)
                .color(Color::Muted)
                .into_any_element();
        }

        match (self.status)(cx) {
            ApiKeyStatus::FromEnvVar(env_var_name) => Label::new(format!("Set via {env_var_name}"))
                .size(LabelSize::Small)
                .color(Color::Muted)
                .into_any_element(),
            ApiKeyStatus::Configured => h_flex()
                .gap_2()
                .items_center()
                .child(
                    Icon::new(IconName::Check)
                        .size(IconSize::Small)
                        .color(Color::Success),
                )
                .child(
                    Label::new("Configured")
                        .size(LabelSize::Small)
                        .color(Color::Muted),
                )
                .child(
                    Button::new("reset-api-key", "Reset")
                        .style(ButtonStyle::Outlined)
                        .label_size(LabelSize::Small)
                        .tab_index(0isize)
                        .on_click(cx.listener(|this, _, window, cx| this.reset(window, cx))),
                )
                .into_any_element(),
            ApiKeyStatus::Unset => v_flex()
                .w_full()
                .gap_1()
                .child(self.render_where_to_find_key())
                .child(
                    h_flex()
                        .w_full()
                        .gap_2()
                        .items_end()
                        .child(
                            div()
                                .flex_1()
                                .min_w_0()
                                .on_action(cx.listener(Self::save))
                                .child(self.input.clone()),
                        )
                        .child(
                            Button::new("save-api-key", "Save")
                                .style(ButtonStyle::Filled)
                                .label_size(LabelSize::Small)
                                .tab_index(0isize)
                                .disabled(self.saving)
                                .on_click(cx.listener(|this, _, window, cx| {
                                    this.save_key(window, cx)
                                })),
                        ),
                )
                .into_any_element(),
        }
    }
}
