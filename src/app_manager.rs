use dioxus::prelude::*;
use rfd::AsyncFileDialog;
use crate::adb::{adb_install, adb_list_packages};
use crate::app::AppState;
use crate::settings::AdbStudioSettings;

#[component]
pub fn AppManager() -> Element {
    let mut logs = use_signal(String::new);
    let mut packages = use_signal(Vec::new);
    let is_loading = use_signal(|| false);
    let app_state = use_context::<Signal<AppState>>();
    let settings = use_context::<Signal<AdbStudioSettings>>();

    let load_packages = move || {
        spawn(async move {
            if let Some(device) = &app_state.read().selected_device {
                let settings_clone = settings.read().clone();
                let serial = device.serial.clone();
                match adb_list_packages(&settings_clone, &serial).await {
                    Ok(output) => {
                        packages.set(output.lines().filter_map(|line| {
                            line.strip_prefix("package:").map(|s| s.to_string())
                        }).collect());
                    },
                    Err(e) => logs.set(e),
                }
            }
        });
    };

    use_effect(move || {
        load_packages();
    });

    let has_device = app_state.read().selected_device.is_some();

    rsx! {
        div {
            h2 { "App Manager" }
            div {
                button { 
                    class: "btn",
                    disabled: !has_device || is_loading(),
                    onclick: move |_| {
                        if !has_device {
                            logs.set("Please select a device first".to_string());
                            return;
                        }
                        let app_state = app_state.clone();
                        let settings = settings.clone();
                        let mut logs = logs.clone();
                        let mut is_loading = is_loading.clone();
                        let load_packages_fn = load_packages.clone();
                        spawn(async move {
                            let device = app_state.read().selected_device.clone();
                            if let Some(device) = device {
                                if let Some(file) = AsyncFileDialog::new().add_filter("apk", &["apk"]).pick_file().await {
                                    is_loading.set(true);
                                    logs.set("Installing APK...".to_string());
                                    let settings_clone = settings.read().clone();
                                    let result = adb_install(&settings_clone, &device.serial, file.path().to_str().unwrap()).await;
                                    is_loading.set(false);
                                    match result {
                                        Ok(_) => {
                                            logs.set("APK installed successfully!".to_string());
                                            load_packages_fn();
                                        },
                                        Err(e) => logs.set(format!("Installation failed: {}", e)),
                                    }
                                }
                            }
                        });
                    },
                    i { class: "fas fa-download" }
                    if is_loading() { "Installing..." } else { "Install APK" }
                }
                button {
                    class: "btn btn-secondary",
                    disabled: !has_device,
                    onclick: move |_| {
                        if !has_device {
                            logs.set("Please select a device first".to_string());
                            return;
                        }
                        let app_state = app_state.clone();
                        let settings = settings.clone();
                        let mut logs = logs.clone();
                        let packages = packages.clone();
                        let load_packages_fn = load_packages.clone();
                        spawn(async move {
                            let device = app_state.read().selected_device.clone();
                            if let Some(device) = device {
                                let pkgs = packages.read().clone();
                                if let Some(pkg) = AsyncFileDialog::new()
                                    .set_title("Select package to uninstall")
                                    .pick_file().await {
                                    if let Some(pkg_name) = pkg.file_name().strip_suffix(".apk") {
                                        logs.set("Uninstalling...".to_string());
                                        let settings_clone = settings.read().clone();
                                        match crate::adb::adb_uninstall(&settings_clone, &device.serial, pkg_name).await {
                                            Ok(_) => {
                                                logs.set(format!("Uninstalled: {}", pkg_name));
                                                load_packages_fn();
                                            },
                                            Err(e) => logs.set(format!("Error: {}", e)),
                                        }
                                    }
                                } else if !pkgs.is_empty() {
                                    logs.set(format!("Select from {} packages", pkgs.len()));
                                }
                            }
                        });
                    },
                    i { class: "fas fa-trash" }
                    "Uninstall App" 
                }
                button { 
                    class: "btn btn-secondary",
                    disabled: !has_device,
                    onclick: move |_| {
                        if has_device {
                            load_packages();
                        } else {
                            logs.set("Please select a device first".to_string());
                        }
                    },
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
