use dioxus::prelude::*;
use rfd::AsyncFileDialog;
use crate::adb::{adb_screenshot, adb_start_screenrecord, adb_stop_screenrecord};
use crate::app::AppState;
use crate::settings::AdbStudioSettings;

#[component]
pub fn MediaManager() -> Element {
    let mut logs = use_signal(String::new);
    let mut is_recording = use_signal(|| false);
    let app_state = use_context::<Signal<AppState>>();
    let settings = use_context::<Signal<AdbStudioSettings>>();

    rsx! {
        div {
            h2 { "Screenshot & Screen Recording" }
            div {
                button { 
                    class: "btn",
                    onclick: move |_| {
                        spawn(async move {
                            if let Some(device) = &app_state.read().selected_device {
                                if let Some(file) = AsyncFileDialog::new().save_file().await {
                                    let settings_clone = settings.read().clone();
                                    let result = adb_screenshot(&settings_clone, &device.serial, file.path().to_str().unwrap()).await;
                                    logs.set(format!("{:?}", result));
                                }
                            } else {
                                logs.set("No device selected.".to_string());
                            }
                        });
                    },
                    i { class: "fas fa-camera" }
                    "Capture Screenshot" 
                }
                button { 
                    class: "btn",
                    onclick: move |_| {
                        spawn(async move {
                            if let Some(device) = &app_state.read().selected_device {
                                let settings_clone = settings.read().clone();
                                if !is_recording() {
                                    let result = adb_start_screenrecord(&settings_clone, &device.serial, "/sdcard/video.mp4").await;
                                    logs.set(format!("{:?}", result));
                                    is_recording.set(true);
                                } else {
                                    let result = adb_stop_screenrecord(&settings_clone, &device.serial).await;
                                    logs.set(format!("{:?}", result));
                                    is_recording.set(false);
                                }
                            } else {
                                logs.set("No device selected.".to_string());
                            }
                        });
                    },
                    i { class: if is_recording() { "fas fa-stop-circle" } else { "fas fa-video" } }
                    if is_recording() { "Stop Recording" } else { "Start Recording" }
                }
            }
            div {
                class: "log-output",
                pre { {logs} }
            }
        }
    }
}
