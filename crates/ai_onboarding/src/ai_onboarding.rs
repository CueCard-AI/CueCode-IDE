mod agent_api_keys_onboarding;
mod agent_panel_onboarding_card;
mod agent_panel_onboarding_content;
mod edit_prediction_onboarding_content;
mod plan_definitions;
mod young_account_banner;

pub use agent_api_keys_onboarding::{ApiKeysWithProviders, ApiKeysWithoutProviders};
pub use agent_panel_onboarding_card::AgentPanelOnboardingCard;
pub use agent_panel_onboarding_content::AgentPanelOnboarding;
use cloud_api_types::Plan;
pub use edit_prediction_onboarding_content::EditPredictionOnboarding;
pub use plan_definitions::PlanDefinitions;
pub use young_account_banner::YoungAccountBanner;

use std::sync::Arc;

use client::{Client, UserStore};
use gpui::{AnyElement, Entity, IntoElement, ParentElement};
use ui::{List, ListBulletItem, RegisterComponent, Tooltip, prelude::*};

#[derive(PartialEq)]
pub enum SignInStatus {
    SignedIn,
    SigningIn,
    SignedOut,
}

impl From<client::Status> for SignInStatus {
    fn from(status: client::Status) -> Self {
        if status.is_signing_in() {
            Self::SigningIn
        } else if status.is_signed_out() {
            Self::SignedOut
        } else {
            Self::SignedIn
        }
    }
}

#[derive(RegisterComponent, IntoElement)]
pub struct ZedAiOnboarding {
    pub sign_in_status: SignInStatus,
    pub plan: Option<Plan>,
    pub account_too_young: bool,
    pub continue_with_zed_ai: Arc<dyn Fn(&mut Window, &mut App)>,
    pub sign_in: Arc<dyn Fn(&mut Window, &mut App)>,
    pub dismiss_onboarding: Option<Arc<dyn Fn(&mut Window, &mut App)>>,
}

impl ZedAiOnboarding {
    pub fn new(
        _client: Arc<Client>,
        user_store: &Entity<UserStore>,
        continue_with_zed_ai: Arc<dyn Fn(&mut Window, &mut App)>,
        cx: &mut App,
    ) -> Self {
        let store = user_store.read(cx);
        let status = *_client.status().borrow();

        Self {
            sign_in_status: status.into(),
            plan: store.plan(),
            account_too_young: store.account_too_young(),
            continue_with_zed_ai,
            sign_in: Arc::new(|_, _| {}),
            dismiss_onboarding: None,
        }
    }

    pub fn with_dismiss(
        mut self,
        dismiss_callback: impl Fn(&mut Window, &mut App) + 'static,
    ) -> Self {
        self.dismiss_onboarding = Some(Arc::new(dismiss_callback));
        self
    }

    fn render_dismiss_button(&self) -> Option<AnyElement> {
        self.dismiss_onboarding.as_ref().map(|dismiss_callback| {
            let callback = dismiss_callback.clone();

            h_flex()
                .absolute()
                .top_0()
                .right_0()
                .child(
                    IconButton::new("dismiss_onboarding", IconName::Close)
                        .icon_size(IconSize::Small)
                        .tooltip(Tooltip::text("Dismiss"))
                        .on_click(move |_, window, cx| {
                            telemetry::event!("Banner Dismissed", source = "AI Onboarding",);
                            callback(window, cx)
                        }),
                )
                .into_any_element()
        })
    }
}

impl RenderOnce for ZedAiOnboarding {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let _ = window;
        let _ = cx;
        v_flex()
            .w_full()
            .relative()
            .gap_1()
            .child(Headline::new("Welcome to CueCode"))
            .child(
                Label::new("Bring your own model (BYOK) — no account required:")
                    .color(Color::Muted)
                    .mb_2(),
            )
            .child(
                List::new()
                    .child(ListBulletItem::new(
                        "Ollama (local): install from ollama.com — default http://localhost:11434",
                    ))
                    .child(ListBulletItem::new(
                        "OpenAI-compatible: Settings → Agent → Language Models → add base URL + key",
                    ))
                    .child(ListBulletItem::new(
                        "Anthropic / OpenAI: set ANTHROPIC_API_KEY or OPENAI_API_KEY, or paste in Settings",
                    ))
            )
            .child(
                Label::new("Skip uses Ollama at localhost with sandbox defaults.")
                    .size(LabelSize::Small)
                    .color(Color::Muted)
                    .mt_1(),
            )
            .child(
                Button::new("continue", "Continue")
                    .full_width()
                    .style(ButtonStyle::Tinted(ui::TintColor::Accent))
                    .on_click({
                        let callback = self.continue_with_zed_ai.clone();
                        move |_, window, cx| callback(window, cx)
                    }),
            )
            .child(
                Label::new("Specs: Planning Hub · `.cursor/specs/` in your repo")
                    .size(LabelSize::Small)
                    .color(Color::Muted),
            )
            .children(self.render_dismiss_button())
            .into_any_element()
    }
}

impl Component for ZedAiOnboarding {
    fn scope() -> ComponentScope {
        ComponentScope::Onboarding
    }

    fn name() -> &'static str {
        "Agent New User Onboarding"
    }

    fn description() -> &'static str {
        "CueCode agent onboarding — connect a local or BYOK model to start."
    }

    fn preview(_window: &mut Window, _cx: &mut App) -> AnyElement {
        div()
            .w_full()
            .min_w_40()
            .max_w(px(1100.))
            .child(
                AgentPanelOnboardingCard::new().child(
                    ZedAiOnboarding {
                        sign_in_status: SignInStatus::SignedOut,
                        plan: None,
                        account_too_young: false,
                        continue_with_zed_ai: Arc::new(|_, _| {}),
                        sign_in: Arc::new(|_, _| {}),
                        dismiss_onboarding: None,
                    }
                    .into_any_element(),
                ),
            )
            .into_any_element()
    }
}
