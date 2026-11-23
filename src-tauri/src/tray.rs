use tauri::{
    menu::{Menu, MenuItem, PredefinedMenuItem},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    AppHandle, Emitter, Manager,
};

/// Shows and focuses the main application window, unminimizing if necessary.
fn show_main_window(app: &AppHandle) {
    if let Some(window) = app.get_webview_window("main") {
        let _ = window.show();
        let _ = window.unminimize();
        let _ = window.set_focus();
    }
}

/// Toggles the visibility of the main application window.
fn toggle_main_window(app: &AppHandle) {
    if let Some(window) = app.get_webview_window("main") {
        if window.is_visible().unwrap_or(false) {
            let _ = window.hide();
        } else {
            let _ = window.show();
            let _ = window.unminimize();
            let _ = window.set_focus();
        }
    }
}

/// Sets up the system tray icon, its context menu, and event handlers for the application.
///
/// Configures:
/// - Tray menu with disabled app title/version, separators, Dashboard, Accounts, and Quit items.
/// - Menu event routing:
///     - "dashboard": opens and focuses the main window.
///     - "accounts": opens and focuses the main window on the accounts tab.
///     - "quit": cleanly exits the application.
/// - Left-click menu presentation on macOS, or window visibility toggling on click events.
pub fn setup(app: &AppHandle) -> anyhow::Result<()> {
    // 1. Build menu items
    let version = &app.package_info().version;
    let title_text = format!("Status Bar v{version}");

    // Disabled title item displaying the application name and current version
    let title_item = MenuItem::with_id(app, "title", &title_text, false, None::<&str>)?;

    // Separators
    let separator1 = PredefinedMenuItem::separator(app)?;
    let separator2 = PredefinedMenuItem::separator(app)?;

    // Navigation and control items
    let dashboard_item = MenuItem::with_id(app, "dashboard", "Dashboard", true, None::<&str>)?;
    let accounts_item = MenuItem::with_id(app, "accounts", "Accounts", true, None::<&str>)?;
    let quit_item = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;

    // Construct the tray context menu
    let menu = Menu::with_items(
        app,
        &[
            &title_item,
            &separator1,
            &dashboard_item,
            &accounts_item,
            &separator2,
            &quit_item,
        ],
    )?;

    // Remove any pre-configured tray icon (e.g., auto-created from tauri.conf.json) to avoid duplicates
    let _ = app.remove_tray_by_id("main-tray");

    // 2. Build the system tray icon using TrayIconBuilder
    let mut builder = TrayIconBuilder::with_id("main-tray")
        .menu(&menu)
        .show_menu_on_left_click(true)
        .icon_as_template(true)
        .tooltip("Status Bar")
        .on_menu_event(|app, event| {
            match event.id().as_ref() {
                "dashboard" => {
                    show_main_window(app);
                    let _ = app.emit("open-tab", "dashboard");
                    let _ = app.emit("navigate", "dashboard");
                }
                "accounts" => {
                    show_main_window(app);
                    let _ = app.emit("open-tab", "accounts");
                    let _ = app.emit("navigate", "accounts");
                }
                "quit" => {
                    app.exit(0);
                }
                _ => {}
            }
        })
        .on_tray_icon_event(|tray, event| {
            // Toggle window visibility on tray icon click (left mouse button release)
            if let TrayIconEvent::Click {
                button: MouseButton::Left,
                button_state: MouseButtonState::Up,
                ..
            } = event
            {
                let app = tray.app_handle();
                toggle_main_window(app);
            }
        });

    // Use default window icon if available
    if let Some(icon) = app.default_window_icon() {
        builder = builder.icon(icon.clone());
    }

    // Register and display the tray icon
    builder.build(app)?;

    Ok(())
}
