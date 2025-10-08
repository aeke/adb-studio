use crate::adb::{
    adb_devices, adb_disconnect, adb_reboot, adb_reboot_bootloader, adb_reboot_recovery,
};
use crate::app_manager::AppManager;
use crate::device::{parse_devices, Device};
use crate::fileops::FileManager;
use crate::log_viewer::LogViewer;
use crate::media::MediaManager;
use crate::settings::{AdbStudioSettings, Settings};
use crate::terminal::Terminal;
use confy::load;
use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
enum View {
    Dashboard,
    Devices,
    Files,
    Terminal,
    Apps,
    Media,
    Logs,
    Settings,
}

#[derive(Clone, PartialEq, Default)]
pub struct AppState {
    pub devices: Vec<Device>,
    pub selected_device: Option<Device>,
}

#[allow(non_snake_case)]
pub fn App() -> Element {
    let mut app_state = use_context_provider(|| Signal::new(AppState::default()));
    let mut current_view = use_signal(|| View::Dashboard);
    let mut settings = use_context_provider(|| {
        Signal::new(load::<AdbStudioSettings>("adb-studio", None).unwrap_or_default())
    });

    use_future(move || async move {
        loop {
            if let Ok(output) = adb_devices(&settings.read()).await {
                app_state.write().devices = parse_devices(&output);
            }
            tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
        }
    });

    rsx! {
        div {
            class: if settings.read().dark_mode { "app" } else { "app light-theme" },
            style: "height: 100vh;",
            div { class: "sidebar",
                h1 { "ADB Studio" }
                nav {
                    button { onclick: move |_| current_view.set(View::Dashboard), i { class: "fas fa-chart-line" } "Dashboard" }
                    button { onclick: move |_| current_view.set(View::Devices), i { class: "fas fa-mobile-alt" } "Devices" }
                    button { onclick: move |_| current_view.set(View::Files), i { class: "fas fa-folder" } "Files" }
                    button { onclick: move |_| current_view.set(View::Terminal), i { class: "fas fa-terminal" } "Terminal" }
                    button { onclick: move |_| current_view.set(View::Apps), i { class: "fas fa-th" } "Apps" }
                    button { onclick: move |_| current_view.set(View::Media), i { class: "fas fa-photo-video" } "Media" }
                    button { onclick: move |_| current_view.set(View::Logs), i { class: "fas fa-file-alt" } "Logs" }
                    button { onclick: move |_| current_view.set(View::Settings), i { class: "fas fa-cog" } "Settings" }
                }
            }
            div { class: "main-content",
                div { class: "top-toolbar",
                    button {
                        onclick: move |_| {
                            let mut current_settings = settings.write();
                            current_settings.dark_mode = !current_settings.dark_mode;
                            let _ = confy::store("adb-studio", None, current_settings.clone());
                        },
                        i { class: if settings.read().dark_mode { "fas fa-moon" } else { "fas fa-sun" } }
                        if settings.read().dark_mode { "Dark" } else { "Light" }
                    }
                    span {
                        i { class: "fas fa-circle", style: "color: var(--success-color); font-size: 0.6rem;" }
                        "{app_state.read().devices.len()} devices"
                    }
                }
                div { class: "content-area",
                    match current_view() {
                        View::Dashboard => rsx!{
                            h2 { "Dashboard" }
                            div { class: "dashboard-grid",
                                div { class: "stat-card",
                                    h3 { "Total Devices" }
                                    div { class: "stat-value", "{app_state.read().devices.len()}" }
                                    div { class: "stat-label", "Connected" }
                                }
                                div { class: "stat-card",
                                    h3 { "Active" }
                                    div { class: "stat-value",
                                        "{app_state.read().devices.iter().filter(|d| d.status == \"device\").count()}"
                                    }
                                    div { class: "stat-label", "Online" }
                                }
                                div { class: "stat-card",
                                    h3 { "Selected" }
                                    div { class: "stat-value",
                                        if app_state.read().selected_device.is_some() { "1" } else { "0" }
                                    }
                                    div { class: "stat-label",
                                        if let Some(dev) = &app_state.read().selected_device {
                                            "{dev.model}"
                                        } else {
                                            "None"
                                        }
                                    }
                                }
                                div { class: "stat-card",
                                    h3 { "ADB Server" }
                                    div { class: "stat-value", i { class: "fas fa-check-circle", style: "color: var(--success-color);" } }
                                    div { class: "stat-label", "Running" }
                                }
                            }
                            if let Some(dev) = &app_state.read().selected_device {
                                div { class: "device-info",
                                    h3 { "Selected Device" }
                                    div { class: "device-info-item",
                                        span { class: "label", "Serial Number" }
                                        span { class: "value", "{dev.serial}" }
                                    }
                                    div { class: "device-info-item",
                                        span { class: "label", "Status" }
                                        span { class: "value", "{dev.status}" }
                                    }
                                    div { class: "device-info-item",
                                        span { class: "label", "Model" }
                                        span { class: "value", "{dev.model}" }
                                    }
                                }
                            }
                            div { class: "quick-actions",
                                div { class: "quick-action-card", onclick: move |_| current_view.set(View::Devices),
                                    i { class: "fas fa-mobile-alt" }
                                    h4 { "Manage Devices" }
                                }
                                div { class: "quick-action-card", onclick: move |_| current_view.set(View::Files),
                                    i { class: "fas fa-folder" }
                                    h4 { "File Transfer" }
                                }
                                div { class: "quick-action-card", onclick: move |_| current_view.set(View::Apps),
                                    i { class: "fas fa-th" }
                                    h4 { "App Manager" }
                                }
                                div { class: "quick-action-card", onclick: move |_| current_view.set(View::Terminal),
                                    i { class: "fas fa-terminal" }
                                    h4 { "Terminal" }
                                }
                                div { class: "quick-action-card", onclick: move |_| current_view.set(View::Media),
                                    i { class: "fas fa-photo-video" }
                                    h4 { "Screenshots" }
                                }
                                div { class: "quick-action-card", onclick: move |_| current_view.set(View::Logs),
                                    i { class: "fas fa-file-alt" }
                                    h4 { "View Logs" }
                                }
                            }
                        },
                        View::Devices => rsx!{
                            h2 { "Connected Devices" }
                            table {
                                thead {
                                    tr {
                                        th { "Serial" }
                                        th { "Status" }
                                        th { "Model" }
                                        th { "Actions" }
                                    }
                                }
                                tbody {
                                    for dev_item in app_state.read().devices.iter().cloned() {
                                        {
                                            let item_clone1 = dev_item.clone();
                                            let item_clone2 = dev_item.clone();
                                            let item_clone3 = dev_item.clone();
                                            let item_clone4 = dev_item.clone();
                                            let is_selected = app_state.read().selected_device.as_ref().map(|d| d.serial == dev_item.serial).unwrap_or(false);
                                            rsx! {
                                                tr {
                                                    class: if is_selected { "selected" } else { "" },
                                                    onclick: move |_| app_state.write().selected_device = Some(dev_item.clone()),
                                                    td { {dev_item.serial.clone()} }
                                                    td { {dev_item.status.clone()} }
                                                    td { {dev_item.model.clone()} }
                                                    td {
                                                        button { onclick: move |_| {
                                                            let serial = item_clone1.serial.clone();
                                                            let settings_clone = settings.read().clone();
                                                            spawn(async move {
                                                                let _ = adb_reboot(&settings_clone, &serial).await;
                                                            });
                                                        }, i { class: "fas fa-redo" } "Reboot" }
                                                        button { onclick: move |_| {
                                                            let serial = item_clone2.serial.clone();
                                                            let settings_clone = settings.read().clone();
                                                            spawn(async move {
                                                                let _ = adb_reboot_recovery(&settings_clone, &serial).await;
                                                            });
                                                        }, i { class: "fas fa-medkit" } "Recovery" }
                                                        button { onclick: move |_| {
                                                            let serial = item_clone3.serial.clone();
                                                            let settings_clone = settings.read().clone();
                                                            spawn(async move {
                                                                let _ = adb_reboot_bootloader(&settings_clone, &serial).await;
                                                            });
                                                        }, i { class: "fas fa-power-off" } "Bootloader" }
                                                        button { onclick: move |_| {
                                                            let serial = item_clone4.serial.clone();
                                                            let settings_clone = settings.read().clone();
                                                            spawn(async move {
                                                                let _ = adb_disconnect(&settings_clone, &serial).await;
                                                            });
                                                        }, i { class: "fas fa-unlink" } "Disconnect" }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        },
                        View::Files => rsx!{ FileManager {} },
                        View::Terminal => rsx!{ Terminal {} },
                        View::Apps => rsx!{ AppManager {} },
                        View::Media => rsx!{ MediaManager {} },
                        View::Logs => rsx!{ LogViewer {} },
                        View::Settings => rsx!{ Settings {} },
                    }
                }
            }
        }
    }
}
