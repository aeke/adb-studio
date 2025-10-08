use dioxus::prelude::*;
use rfd::AsyncFileDialog;
use crate::adb::{adb_install, adb_list_packages};
use crate::app::AppState;
use crate::settings::AdbStudioSettings;

#[component]
pub fn AppManager() -> Element {
    let mut logs = use_signal(String::new);
    let mut packages = use_signal(Vec::new);
    let app_state = use_context::<Signal<AppState>>();
    let settings = use_context::<Signal<AdbStudioSettings>>();

    let mut list_packages = use_resource(move || async move {
        if let Some(device) = &app_state.read().selected_device {
            let settings_clone = settings.read().clone();
            match adb_list_packages(&settings_clone, &device.serial).await {
                Ok(output) => {
                    packages.set(output.lines().filter_map(|line| {
                        line.strip_prefix("package:").map(|s| s.to_string())
                    }).collect());
                },
                Err(e) => logs.set(e),
            }
        } else {
            logs.set("No device selected.".to_string());
        }
    });

    rsx! {
        div {
            h2 { "App Manager" }
            div {
                button { 
                    class: "btn",
                    onclick: move |_| {
                        spawn(async move {
                            if let Some(device) = &app_state.read().selected_device {
                                if let Some(file) = AsyncFileDialog::new().add_filter("apk", &["apk"]).pick_file().await {
                                    let settings_clone = settings.read().clone();
                                    let result = adb_install(&settings_clone, &device.serial, file.path().to_str().unwrap()).await;
                                    logs.set(format!("{:?}", result));
                                }
                            } else {
                                logs.set("No device selected.".to_string());
                            }
                        });
                    },
                    i { class: "fas fa-download" }
                    "Install APK" 
                }
                button {
                    class: "btn btn-secondary",
                    onclick: move |_| {
                        spawn(async move {
                            if let Some(_device) = &app_state.read().selected_device {
                                logs.set("Uninstall functionality not yet implemented.".to_string());
                            } else {
                                logs.set("No device selected.".to_string());
                            }
                        });
                    },
                    i { class: "fas fa-trash" }
                    "Uninstall App" 
                }
                button { 
                    class: "btn btn-secondary",
                    onclick: move |_| list_packages.restart(),
                    i { class: "fas fa-sync" }
                    "Refresh" 
                }
            }
            div {
                h3 { "Installed Packages" }
                ul {
                    for package in packages() {
                        li { "{package}" }
                    }
                }
            }
            div {
                class: "log-output",
                pre { {logs} }
            }
        }
    }
}
