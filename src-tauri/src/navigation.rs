use tauri::Url;

#[derive(Debug, PartialEq)]
/// A popup can stay in the app, open in the browser, or be blocked.
pub enum PopupDestination {
    Internal,
    External,
    Blocked,
}

/// Choose where a new window can open based on its address.
pub fn popup_destination(url: &Url) -> PopupDestination {
    if url.as_str() == "about:blank" {
        return PopupDestination::Internal;
    }

    let is_google = matches!(
        url.host_str(),
        Some("accounts.google.com" | "messages.google.com" | "google.com" | "www.google.com")
    );
    if url.scheme() == "https" && url.port_or_known_default() == Some(443) && is_google {
        return PopupDestination::Internal;
    }

    match url.scheme() {
        "http" | "https" => PopupDestination::External,
        _ => PopupDestination::Blocked,
    }
}
