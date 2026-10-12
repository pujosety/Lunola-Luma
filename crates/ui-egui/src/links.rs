//! Generic platform-aware URL opening helper. The UI does not expose external community/help links.

use serde_json::{Value, json};

use crate::PhotocraftApp;

/// Open `url` in the system browser (a new tab on the web) and note it in the status bar. Prefers
/// the platform `open_url` service (reliable on Windows/macOS/Linux); falls back to `ctx.open_url`.
pub fn open(app: &mut PhotocraftApp, ctx: &egui::Context, url: &str) -> Value {
    let opened = app.services.open_url.as_ref().map(|f| f(url).is_ok()).unwrap_or(false);
    if !opened {
        ctx.open_url(egui::OpenUrl::new_tab(url));
    }
    let template = "Opened {url}";
    app.ui.status = crate::i18n::fmt(tl!(template), &[("url", url)]);
    json!({"url": url})
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generic_open_uses_the_platform_service() {
        use std::sync::{Arc, Mutex};
        let opened: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(Vec::new()));
        let rec = opened.clone();
        let services = crate::Services {
            open_url: Some(Box::new(move |u: &str| {
                rec.lock().unwrap().push(u.to_string());
                Ok(())
            })),
            ..Default::default()
        };
        let mut app = PhotocraftApp::new(photocraft_engine::Session::new(), services);
        let ctx = egui::Context::default();
        let url = "https://example.invalid";
        let result = open(&mut app, &ctx, url);
        assert_eq!(result["url"], url);
        assert_eq!(app.ui.status, format!("Opened {url}"));
        assert_eq!(opened.lock().unwrap().as_slice(), [url]);
    }

    #[test]
    fn help_menu_is_absent_and_about_remains_available() {
        let app = PhotocraftApp::new(photocraft_engine::Session::new(), Default::default());
        let items = crate::menus::menu_items(&app);
        assert!(!crate::menus::TOP_MENUS.contains(&"Help"));
        assert!(!items.iter().any(|i| i.path.first().is_some_and(|p| p == "Help")));
        assert!(items.iter().any(|i| i.id == "help.about" && i.path == ["File"]));
        assert!(items.iter().any(|i| i.id == "help.systemInfo" && i.path.is_empty()));
    }
}
