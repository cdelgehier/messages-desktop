mod navigation;

use navigation::{popup_destination, PopupDestination};
use std::sync::atomic::{AtomicU64, Ordering};
use tauri::{
    webview::{NewWindowFeatures, NewWindowResponse},
    App, AppHandle, Manager, Url, WebviewUrl, WebviewWindow, WebviewWindowBuilder, Wry,
};
use tauri_plugin_opener::OpenerExt;

#[cfg(target_os = "macos")]
const SAFARI_UA: &str = "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/26.0 Safari/605.1.15";

static NEXT_POPUP_ID: AtomicU64 = AtomicU64::new(1);

/// Build a webview with the same browser identity and popup handling.
fn webview_builder<'a>(
    app: &'a AppHandle,
    label: &str,
    url: Url,
) -> WebviewWindowBuilder<'a, Wry, AppHandle> {
    let popup_app = app.clone();
    let builder = WebviewWindowBuilder::new(app, label, WebviewUrl::External(url))
        .on_new_window(move |url, features| open_popup(&popup_app, url, features));

    // Match the WebKit engine. Keep navigator properties unchanged.
    #[cfg(target_os = "macos")]
    let builder = builder.user_agent(SAFARI_UA);

    builder
}

/// Create a login window using the opener's browser configuration and session.
fn create_login_window(
    app: &AppHandle,
    features: NewWindowFeatures,
) -> tauri::Result<WebviewWindow> {
    let id = NEXT_POPUP_ID.fetch_add(1, Ordering::Relaxed);
    let label = format!("google-login-{id}");
    let mut builder = webview_builder(app, &label, "about:blank".parse().unwrap())
        // WebKit requires this configuration to keep window.opener and cookies.
        .window_features(features)
        .title("Google sign-in — Messages")
        .inner_size(520.0, 720.0)
        .min_inner_size(400.0, 500.0)
        .center();

    if let Some(main) = app.get_webview_window("main") {
        builder = builder.parent(&main)?;
    }

    builder.build()
}

/// Keep Google login in the app and send other web links to the browser.
fn open_popup(app: &AppHandle, url: Url, features: NewWindowFeatures) -> NewWindowResponse<Wry> {
    match popup_destination(&url) {
        PopupDestination::Internal => match create_login_window(app, features) {
            Ok(window) => NewWindowResponse::Create { window },
            Err(error) => {
                eprintln!("Could not open the sign-in window: {error}");
                NewWindowResponse::Deny
            }
        },
        PopupDestination::External => {
            if app.opener().open_url(url.as_str(), None::<&str>).is_err() {
                eprintln!("Could not open the link in the default browser");
            }
            NewWindowResponse::Deny
        }
        PopupDestination::Blocked => NewWindowResponse::Deny,
    }
}

/// Open Messages in the main Tauri window.
fn setup(app: &mut App) -> Result<(), Box<dyn std::error::Error>> {
    webview_builder(
        app.handle(),
        "main",
        "https://messages.google.com/web".parse()?,
    )
    .title("Messages")
    .inner_size(1200.0, 800.0)
    .min_inner_size(800.0, 600.0)
    .build()?;
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        // Route popup links in Rust so Google login stays in the shared session.
        .plugin(
            tauri_plugin_opener::Builder::new()
                .open_js_links_on_click(false)
                .build(),
        )
        .setup(setup)
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
