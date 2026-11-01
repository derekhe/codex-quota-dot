mod autostart;
mod usage;

use tauri::{
    menu::{CheckMenuItem, Menu, MenuItem},
    tray::TrayIconBuilder,
    Manager,
};
use usage::CodexUsageSnapshot;

const AUTOSTART_LABEL: &str = "开机自启动 / Start at login";
const AUTOSTART_FAILED_LABEL: &str = "开机自启动 / Start at login (失败，请重试 / Failed, retry)";

#[tauri::command]
async fn get_usage_snapshot() -> Result<CodexUsageSnapshot, String> {
    tauri::async_runtime::spawn_blocking(usage::fetch_usage)
        .await
        .map_err(|_| "The local usage provider stopped unexpectedly".to_string())?
        .map_err(|error| error.to_string())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let autostart_manager = autostart::manager()?;
            let show = MenuItem::with_id(app, "show", "Show quota", true, None::<&str>)?;
            // Read the OS registration rather than maintaining a second local setting.
            // Merely launching the app never enables autostart.
            let autostart_state = autostart_manager.is_enabled();
            let autostart = CheckMenuItem::with_id(
                app,
                "autostart",
                if autostart_state.is_ok() {
                    AUTOSTART_LABEL
                } else {
                    AUTOSTART_FAILED_LABEL
                },
                true,
                autostart_state.unwrap_or(false),
                None::<&str>,
            )?;
            let quit = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
            let menu = Menu::with_items(app, &[&show, &autostart, &quit])?;
            TrayIconBuilder::new()
                .icon(
                    app.default_window_icon()
                        .cloned()
                        .ok_or("application icon is missing")?,
                )
                .menu(&menu)
                .tooltip("Codex Quota Dot")
                .on_menu_event(move |app, event| match event.id.as_ref() {
                    "show" => {
                        if let Some(window) = app.get_webview_window("main") {
                            let _ = window.show();
                            let _ = window.set_focus();
                        }
                    }
                    "autostart" => {
                        // Native check menu items toggle before their event is delivered.
                        if let Ok(enabled) = autostart.is_checked() {
                            let manager = &autostart_manager;
                            let result = if enabled {
                                manager.enable()
                            } else {
                                manager.disable()
                            };
                            let actual = manager.is_enabled();
                            let succeeded = result.is_ok()
                                && actual.as_ref().is_ok_and(|value| *value == enabled);
                            let _ = autostart.set_checked(actual.unwrap_or(!enabled));
                            let _ = autostart.set_text(if succeeded {
                                AUTOSTART_LABEL
                            } else {
                                AUTOSTART_FAILED_LABEL
                            });
                        }
                    }
                    "quit" => app.exit(0),
                    _ => {}
                })
                .build(app)?;
            Ok(())
        })
        .on_window_event(|window, event| {
            if let tauri::WindowEvent::CloseRequested { api, .. } = event {
                api.prevent_close();
                let _ = window.hide();
            }
        })
        .invoke_handler(tauri::generate_handler![get_usage_snapshot])
        .run(tauri::generate_context!())
        .expect("error while running Codex Quota Dot");
}
