use dioxus::prelude::*;
use rfd::AsyncFileDialog;
use crate::adb::{adb_push, adb_list_packages};
use crate::app::AppState;
use crate::settings::AdbStudioSettings;

#[component]
pub fn AppManager() -> Element {
    let mut logs = use_signal(String::new);
    let mut packages = use_signal(Vec::new);
    let mut selected_package = use_signal(|| None::<String>);
    let mut search_query = use_signal(String::new);
    let mut is_loading = use_signal(|| false);
    let mut install_progress = use_signal(|| 0);
    let mut alert_message = use_signal(|| String::new());
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
    let filtered_packages = packages().into_iter()
        .filter(|p| p.to_lowercase().contains(&search_query().to_lowercase()))
        .collect::<Vec<_>>();

    rsx! {
        if !alert_message.read().is_empty() {
            div { 
                class: "alert-message",
                style: "background: var(--success-color);",
                "{alert_message}" 
            }
        }
        div {
            h2 { "App Manager" }
            if install_progress() > 0 {
                div {
                    style: "margin: 16px 0; padding: 16px; background: var(--card-bg); border: 1px solid var(--border-color); border-radius: 8px; max-width: 100%;",
                    h3 { style: "margin: 0 0 12px 0; font-size: 1rem;", "Installing APK" }
                    div {
                        style: "background: var(--tertiary-bg); border-radius: 6px; height: 24px; overflow: hidden;",
                        div {
                            style: format!("background: var(--accent-color); height: 100%; width: {}%; transition: width 0.3s ease;", install_progress()),
                        }
                    }
                    div {
                        style: "text-align: center; margin-top: 8px; font-size: 0.875rem; color: var(--secondary-text); font-weight: 500;",
                        "{install_progress()}%"
                    }
                }
            }
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
                        let mut install_progress = install_progress.clone();
                        let load_packages_fn = load_packages.clone();
                        spawn(async move {
                            let device = app_state.read().selected_device.clone();
                            if let Some(device) = device {
                                if let Some(file) = AsyncFileDialog::new().add_filter("apk", &["apk"]).pick_file().await {
                                    is_loading.set(true);
                                    install_progress.set(10);
                                    logs.set("Pushing APK to device...".to_string());
                                    let settings_clone = settings.read().clone();
                                    let apk_path = file.path().to_str().unwrap();
                                    let remote_path = "/data/local/tmp/temp.apk";
                                    
                                    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
                                    install_progress.set(30);
                                    match adb_push(&settings_clone, &device.serial, apk_path, remote_path).await {
                                        Ok(_) => {
                                            install_progress.set(60);
                                            logs.set("Installing APK...".to_string());
                                            let install_cmd = format!("pm install -r {}", remote_path);
                                            match crate::adb::adb_shell(&settings_clone, &install_cmd).await {
                                                Ok(output) => {
                                                    install_progress.set(100);
                                                    if output.contains("Success") {
                                                        logs.set("APK installed successfully!".to_string());
                                                        load_packages_fn();
                                                        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
                                                        install_progress.set(0);
                                                        alert_message.set("APK installed successfully!".to_string());
                                                        let mut alert_clone = alert_message.clone();
                                                        spawn(async move {
                                                            tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;
                                                            alert_clone.set(String::new());
                                                        });
                                                    } else {
                                                        logs.set(format!("Installation failed: {}", output));
                                                        install_progress.set(0);
                                                    }
                                                },
                                                Err(e) => {
                                                    install_progress.set(0);
                                                    logs.set(format!("Installation failed: {}", e));
                                                }
                                            }
                                        },
                                        Err(e) => {
                                            install_progress.set(0);
                                            logs.set(format!("Push failed: {}", e));
                                        }
                                    }
                                    is_loading.set(false);
                                }
                            }
                        });
                    },
                    i { class: "fas fa-download" }
                    if is_loading() { "Installing..." } else { "Install APK" }
                }
                button {
                    class: "btn btn-secondary",
                    disabled: !has_device || selected_package().is_none(),
                    onclick: move |_| {
                        if !has_device {
                            logs.set("Please select a device first".to_string());
                            return;
                        }
                        if let Some(pkg_name) = selected_package() {
                            let app_state = app_state.clone();
                            let settings = settings.clone();
                            let mut logs = logs.clone();
                            let load_packages_fn = load_packages.clone();
                            let mut selected_package = selected_package.clone();
                            spawn(async move {
                                let device = app_state.read().selected_device.clone();
                                if let Some(device) = device {
                                    logs.set(format!("Uninstalling {}...", pkg_name));
                                    let settings_clone = settings.read().clone();
                                    match crate::adb::adb_uninstall(&settings_clone, &device.serial, &pkg_name).await {
                                        Ok(_) => {
                                            logs.set(format!("Uninstalled: {}", pkg_name));
                                            selected_package.set(None);
                                            load_packages_fn();
                                            alert_message.set(format!("Uninstalled: {}", pkg_name));
                                            search_query.set(String::new());
                                            let mut alert_clone = alert_message.clone();
                                            spawn(async move {
                                                tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;
                                                alert_clone.set(String::new());
                                            });
                                        },
                                        Err(e) => logs.set(format!("Error: {}", e)),
                                    }
                                }
                            });
                        } else {
                            logs.set("Please select a package from the list".to_string());
                        }
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
                style: "margin-bottom: 12px;",
                input {
                    r#type: "text",
                    placeholder: "Search packages...",
                    autocomplete: "off",
                    spellcheck: "false",
                    value: "{search_query}",
                    oninput: move |e| search_query.set(e.value().clone()),
                    style: "max-width: 300px;"
                }
            }
            div {
                h3 { "Installed Packages ({filtered_packages.len()})" }
                if filtered_packages.is_empty() && !packages().is_empty() {
                    div {
                        style: "padding: 20px; text-align: center; color: var(--secondary-text); font-size: 0.875rem;",
                        "No packages found matching \"{search_query}\""
                    }
                }
                ul {
                    style: "list-style: none; padding: 0; padding-bottom: 50px; max-height: calc(100vh - 280px); overflow-y: auto;",
                    for package in filtered_packages {
                        li { 
                            class: if selected_package() == Some(package.clone()) { "package-item selected" } else { "package-item" },
                            onclick: move |_| {
                                selected_package.set(Some(package.clone()));
                            },
                            "{package}" 
                        }
                    }
                }
            }
        }
    }
}
