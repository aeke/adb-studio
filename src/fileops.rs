use crate::adb::{adb_push, adb_pull};
use dioxus::prelude::*;
use rfd::AsyncFileDialog;
use crate::app::AppState;
use crate::settings::AdbStudioSettings;

#[component]
pub fn FileManager() -> Element {
    let mut logs = use_signal(String::new);
    let app_state = use_context::<Signal<AppState>>();
    let settings = use_context::<Signal<AdbStudioSettings>>();

    rsx! {
        div {
            h2 { "File Manager" }
            div {
                button { 
                    class: "btn",
                    onclick: move |_| {
                        spawn(async move {
                            if let Some(device) = &app_state.read().selected_device {
                                if let Some(file) = AsyncFileDialog::new().pick_file().await {
                                    let settings_clone = settings.read().clone();
                                    let result = adb_push(&settings_clone, &device.serial, file.path().to_str().unwrap(), "/sdcard/").await;
                                    logs.set(format!("{:?}", result));
                                }
                            } else {
                                logs.set("No device selected.".to_string());
                            }
                        });
                    },
                    i { class: "fas fa-upload" }
                    "Push File" 
                }
                button { 
                    class: "btn",
                    onclick: move |_| {
                        spawn(async move {
                            if let Some(device) = &app_state.read().selected_device {
                                if let Some(folder) = AsyncFileDialog::new().pick_folder().await {
                                    let settings_clone = settings.read().clone();
                                    let result = adb_pull(&settings_clone, &device.serial, "/sdcard/", folder.path().to_str().unwrap()).await;
                                    logs.set(format!("{:?}", result));
                                }
                            } else {
                                logs.set("No device selected.".to_string());
                            }
                        });
                    },
                    i { class: "fas fa-download" }
                    "Pull File" 
                }
            }
            div {
                class: "log-output",
                pre { {logs} }
            }
        }
    }
}
