use std::{sync::Mutex, time::Duration};
use serde::Serialize;
use tauri::{
    ipc::Channel,
    menu::{Menu, MenuItem},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    AppHandle, Manager, State,
};
use tauri_plugin_notification::NotificationExt;
use tauri_plugin_updater::{Update, UpdaterExt};

struct PendingUpdate(Mutex<Option<Update>>);

#[derive(Debug, thiserror::Error)]
enum UpdateError {
    #[error(transparent)]
    Updater(#[from] tauri_plugin_updater::Error),
    #[error("Bekleyen bir güncelleme bulunamadı.")]
    NoPendingUpdate,
}

impl Serialize for UpdateError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where S: serde::Serializer {
        serializer.serialize_str(&self.to_string())
    }
}

type UpdateResult<T> = std::result::Result<T, UpdateError>;

#[derive(Clone, Serialize)]
#[serde(tag = "event", content = "data")]
enum DownloadEvent {
    #[serde(rename_all = "camelCase")]
    Started { content_length: Option<u64> },
    #[serde(rename_all = "camelCase")]
    Progress { chunk_length: usize },
    Finished,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct UpdateMetadata {
    version: String,
    current_version: String,
    notes: Option<String>,
    date: Option<String>,
}

#[tauri::command]
async fn check_for_update(
    app: AppHandle,
    pending_update: State<'_, PendingUpdate>,
) -> UpdateResult<Option<UpdateMetadata>> {
    let update = app.updater()?.check().await?;
    let metadata = update.as_ref().map(|item| UpdateMetadata {
        version: item.version.clone(),
        current_version: item.current_version.clone(),
        notes: item.body.clone(),
        date: item.date.map(|d| d.to_string()),
    });
    *pending_update.0.lock().expect("updater state poisoned") = update;
    Ok(metadata)
}

#[tauri::command]
async fn install_pending_update(
    pending_update: State<'_, PendingUpdate>,
    on_event: Channel<DownloadEvent>,
) -> UpdateResult<()> {
    let Some(update) = pending_update.0.lock().expect("updater state poisoned").take() else {
        return Err(UpdateError::NoPendingUpdate);
    };

    let mut started = false;
    update
        .download_and_install(
            |chunk_length, content_length| {
                if !started {
                    let _ = on_event.send(DownloadEvent::Started { content_length });
                    started = true;
                }
                let _ = on_event.send(DownloadEvent::Progress { chunk_length });
            },
            || {
                let _ = on_event.send(DownloadEvent::Finished);
            },
        )
        .await?;
    Ok(())
}

fn show_main(app: &tauri::AppHandle) {
    if let Some(window) = app.get_webview_window("main") {
        let _ = window.show();
        let _ = window.unminimize();
        let _ = window.set_focus();
    }
}

fn open_update_screen(app: &tauri::AppHandle) {
    show_main(app);
    if let Some(window) = app.get_webview_window("main") {
        let _ = window.eval("window.location.href='tauri://localhost/index.html?manual=1'");
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_window_state::Builder::default().build())
        .setup(|app| {
            #[cfg(desktop)]
            {
                app.handle().plugin(tauri_plugin_updater::Builder::new().build())?;
                app.manage(PendingUpdate(Mutex::new(None)));
            }

            let show = MenuItem::with_id(app, "show", "BARON'u Aç", true, None::<&str>)?;
            let updates = MenuItem::with_id(app, "updates", "Güncellemeleri Kontrol Et", true, None::<&str>)?;
            let quit = MenuItem::with_id(app, "quit", "Çıkış", true, None::<&str>)?;
            let menu = Menu::with_items(app, &[&show, &updates, &quit])?;

            TrayIconBuilder::with_id("baron-tray")
                .icon(app.default_window_icon().cloned().expect("BARON simgesi bulunamadı"))
                .tooltip("BARON Ders Programı")
                .menu(&menu)
                .show_menu_on_left_click(false)
                .on_menu_event(|app, event| match event.id.as_ref() {
                    "show" => show_main(app),
                    "updates" => open_update_screen(app),
                    "quit" => app.exit(0),
                    _ => {}
                })
                .on_tray_icon_event(|tray, event| {
                    if let TrayIconEvent::Click {
                        button: MouseButton::Left,
                        button_state: MouseButtonState::Up,
                        ..
                    } = event
                    {
                        show_main(tray.app_handle());
                    }
                })
                .build(app)?;

            let handle = app.handle().clone();
            std::thread::spawn(move || {
                std::thread::sleep(Duration::from_millis(2200));
                if let Some(splash) = handle.get_webview_window("splashscreen") {
                    let _ = splash.close();
                }
                show_main(&handle);
            });

            if let Some(main) = app.get_webview_window("main") {
                let app_handle = app.handle().clone();
                main.on_window_event(move |event| {
                    if let tauri::WindowEvent::CloseRequested { api, .. } = event {
                        api.prevent_close();
                        if let Some(window) = app_handle.get_webview_window("main") {
                            let _ = window.hide();
                        }
                    }
                });
            }

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![check_for_update, install_pending_update])
        .run(tauri::generate_context!())
        .expect("BARON uygulaması başlatılamadı");
}
