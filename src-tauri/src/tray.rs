use crate::state::AppState;
use tauri::{
    image::Image,
    menu::{CheckMenuItem, Menu, MenuItem},
    tray::{MouseButton, MouseButtonState, TrayIcon, TrayIconBuilder, TrayIconEvent},
    AppHandle, Emitter, Listener, Manager, Wry,
};

pub fn create_tray(app: &AppHandle) -> tauri::Result<TrayIcon<Wry>> {
    let toggle_auto_send = CheckMenuItem::with_id(
        app,
        "toggle_auto_send",
        "Auto-Send",
        true,
        false,
        None::<&str>,
    )?;
    let toggle_auto_receive = CheckMenuItem::with_id(
        app,
        "toggle_auto_receive",
        "Auto-Receive",
        true,
        false,
        None::<&str>,
    )?;
    let quit_i = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
    let show_i = MenuItem::with_id(app, "show", "Show Window", true, None::<&str>)?;

    let menu = Menu::with_items(
        app,
        &[
            &show_i,
            &MenuItem::with_id(app, "sep1", "-", true, None::<&str>)?,
            &toggle_auto_send,
            &toggle_auto_receive,
            &MenuItem::with_id(app, "sep2", "-", true, None::<&str>)?,
            &quit_i,
        ],
    )?;

    // Initial state sync
    let state = app.state::<AppState>();
    let settings = state.settings.lock().unwrap();
    let _ = toggle_auto_send.set_checked(settings.auto_send);
    let _ = toggle_auto_receive.set_checked(settings.auto_receive);

    // Initial Icon Selection
    let (icon, is_template) = get_platform_icon(app);

    // Build Tray
    let tray = TrayIconBuilder::with_id("main-tray")
        .menu(&menu)
        .show_menu_on_left_click(false)
        .icon(icon)
        .icon_as_template(is_template)
        .on_menu_event(move |app: &AppHandle, event| {
            let id = event.id.as_ref();
            match id {
                "quit" => app.exit(0),
                "show" => {
                    if let Some(window) = app.get_webview_window("main") {
                        let _ = window.show();
                        let _ = window.set_focus();
                        set_badge(app, false);
                    }
                }
                "toggle_auto_send" => {
                    let state = app.state::<AppState>();
                    let mut settings = state.settings.lock().unwrap();
                    settings.auto_send = !settings.auto_send;
                    crate::storage::save_settings(app, &settings);
                    let _ = app.emit("settings-changed", settings.clone());
                }
                "toggle_auto_receive" => {
                    let state = app.state::<AppState>();
                    let mut settings = state.settings.lock().unwrap();
                    settings.auto_receive = !settings.auto_receive;
                    crate::storage::save_settings(app, &settings);
                    let _ = app.emit("settings-changed", settings.clone());
                }
                _ => {}
            }
        })
        .on_tray_icon_event(|tray: &TrayIcon<Wry>, event| {
            if let TrayIconEvent::Click {
                button: MouseButton::Left,
                button_state: MouseButtonState::Up,
                ..
            } = event
            {
                let app = tray.app_handle();
                if let Some(window) = app.get_webview_window("main") {
                    let _ = window.show();
                    let _ = window.set_focus();
                    set_badge(app, false);
                }
            }
        })
        .build(app)?;

    // Setup Theme Listener
    let listener_handle = app.clone();
    app.listen("tauri://theme-changed", move |_event| {
        update_tray_icon(&listener_handle);
    });

    Ok(tray)
}

fn get_platform_icon(app: &AppHandle) -> (Image<'static>, bool) {
    #[cfg(target_os = "macos")]
    {
        (
            tauri::image::Image::from_bytes(include_bytes!(
                "../icons/pdf/clustercut-tray.Template.pdf"
            ))
            .expect("Failed to load macOS tray icon"),
            true,
        )
    }

    #[cfg(target_os = "windows")]
    {
        (
            tauri::image::Image::from_bytes(include_bytes!("../icons/ico/clustercut-tray.ico"))
                .expect("Failed to load Windows tray icon"),
            false,
        )
    }

    #[cfg(target_os = "linux")]
    {
        get_linux_icon(app)
    }

    #[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "linux")))]
    {
        (app.default_window_icon().unwrap().clone(), false)
    }
}

#[cfg(target_os = "linux")]
fn get_linux_icon(app: &AppHandle) -> (Image<'static>, bool) {
    use tauri::Theme;

    // Attempt to detect theme via Main Window
    let theme = if let Some(window) = app.get_webview_window("main") {
        match window.theme() {
            Ok(t) => t,
            Err(_) => Theme::Light, // Fallback
        }
    } else {
        Theme::Light // Fallback if no window
    };

    tracing::info!("Detected Linux System Theme: {:?}", theme);

    match theme {
        Theme::Dark => (
            tauri::image::Image::from_bytes(include_bytes!(
                "../icons/png/clustercut-tray-white.png"
            ))
            .expect("Failed to load White tray icon"),
            false,
        ),
        Theme::Light => (
            tauri::image::Image::from_bytes(include_bytes!(
                "../icons/png/clustercut-tray-black.png"
            ))
            .expect("Failed to load Black tray icon"),
            false,
        ),
        _ => (
            tauri::image::Image::from_bytes(include_bytes!("../icons/png/clustercut-tray.png"))
                .expect("Failed to load Default tray icon"),
            false,
        ),
    }
}

pub fn update_tray_icon(app: &AppHandle) {
    #[cfg(target_os = "linux")]
    {
        if let Some(tray) = app.tray_by_id("main-tray") {
            let (icon, _is_template) = get_linux_icon(app);
            let _ = tray.set_icon(Some(icon));
        }
    }
}

pub fn update_tray_menu(_app: &AppHandle) {
    // STUB
}

pub fn set_badge(_app: &AppHandle, _show: bool) {
    // STUB
}
