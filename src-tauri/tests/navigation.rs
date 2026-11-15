#[path = "../src/navigation.rs"]
mod navigation;

use navigation::{popup_destination, PopupDestination};
use tauri::Url;

#[test]
fn keeps_google_login_windows_in_the_app() {
    for address in [
        "https://accounts.google.com/v3/signin/identifier?service=messages",
        "https://messages.google.com/web/",
        "https://google.com/",
        "https://www.google.com/",
        "about:blank",
    ] {
        let url = Url::parse(address).unwrap();
        assert_eq!(
            popup_destination(&url),
            PopupDestination::Internal,
            "{address}"
        );
    }
}

#[test]
fn opens_other_web_links_in_the_default_browser() {
    for address in [
        "https://example.com/",
        "http://example.com/",
        "https://accounts.google.com.example.com/",
        "https://accounts.google.com@evil.example/",
        "https://evil.example/?next=https://accounts.google.com/",
        "http://accounts.google.com/",
        "https://accounts.google.com:8443/",
    ] {
        let url = Url::parse(address).unwrap();
        assert_eq!(
            popup_destination(&url),
            PopupDestination::External,
            "{address}"
        );
    }
}

#[test]
fn blocks_popups_that_can_open_files_or_run_code() {
    for address in [
        "file:///etc/hosts",
        "javascript:alert(1)",
        "data:text/html,test",
        "about:config",
        "custom-app://open",
    ] {
        let url = Url::parse(address).unwrap();
        assert_eq!(
            popup_destination(&url),
            PopupDestination::Blocked,
            "{address}"
        );
    }
}
