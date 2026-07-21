use client::app_urls::APP_URL_SCHEMES;
use gpui::{AsyncApp, actions};

actions!(
    cli,
    [
        #[action(deprecated_aliases = ["cli::RegisterZedScheme"])]
        /// Registers the cuecode:// URL scheme handler (legacy zed:// links are normalized in-app).
        RegisterAppUrlSchemes
    ]
);

pub async fn register_zed_scheme(cx: &AsyncApp) -> anyhow::Result<()> {
    register_app_url_schemes(cx).await
}

pub async fn register_app_url_schemes(cx: &AsyncApp) -> anyhow::Result<()> {
    for scheme in APP_URL_SCHEMES {
        cx.update(|cx| cx.register_url_scheme(scheme)).await?;
    }
    Ok(())
}
