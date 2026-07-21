use gpui::IntoElement;
use ui::prelude::*;

#[derive(IntoElement)]
pub struct YoungAccountBanner;

impl RenderOnce for YoungAccountBanner {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        // CueCode v1: cloud trial eligibility banner disabled.
        div()
    }
}
