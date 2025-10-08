use dioxus::prelude::*;
use confy::{load, store};
use serde::{Serialize, Deserialize};

#[derive(Default, Serialize, Deserialize, Clone)]
pub struct AdbStudioSettings {
    pub adb_path: String,
    pub dark_mode: bool,
}

#[component]
pub fn Settings() -> Element {
    let mut settings = use_signal(|| load::<AdbStudioSettings>("adb-studio", None).unwrap_or_default());

    rsx! {
        div {
            h2 { "Settings" }
            div { class: "form-group",
                label { "ADB Binary Path" }
                input {
                    r#type: "text",
                    value: "{settings.read().adb_path}",
                    placeholder: "Leave empty to use system ADB",
                    oninput: move |event| {
                        settings.write().adb_path = event.value().clone();
                        let _ = store("adb-studio", None, settings.read().clone());
                    },
                }
            }
            div { class: "form-group",
                label { 
                    "Dark Mode"
                    input {
                        r#type: "checkbox",
                        checked: "{settings.read().dark_mode}",
                        oninput: move |event| {
                            settings.write().dark_mode = event.checked();
                            let _ = store("adb-studio", None, settings.read().clone());
                        },
                    }
                }
            }
        }
    }
}