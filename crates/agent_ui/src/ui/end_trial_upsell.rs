use std::sync::Arc;

use gpui::{AnyElement, App, IntoElement, RenderOnce, Window};
use ui::prelude::*;

#[derive(IntoElement, RegisterComponent)]
pub struct EndTrialUpsell {
    dismiss_upsell: Arc<dyn Fn(&mut Window, &mut App)>,
}

impl EndTrialUpsell {
    pub fn new(dismiss_upsell: Arc<dyn Fn(&mut Window, &mut App)>) -> Self {
        Self { dismiss_upsell }
    }
}

impl RenderOnce for EndTrialUpsell {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let _ = self.dismiss_upsell;
        // CueCode v1: cloud trial upsell disabled.
        div()
    }
}

impl Component for EndTrialUpsell {
    fn scope() -> ComponentScope {
        ComponentScope::Onboarding
    }

    fn name() -> &'static str {
        "End of Trial Upsell Banner"
    }

    fn sort_name() -> &'static str {
        "End of Trial Upsell Banner"
    }

    fn description() -> &'static str {
        "Disabled in CueCode — no cloud trial upsell."
    }

    fn preview(_window: &mut Window, _cx: &mut App) -> AnyElement {
        div().into_any_element()
    }
}
