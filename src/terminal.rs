use crate::adb::adb_shell;
use dioxus::prelude::*;
use crate::settings::AdbStudioSettings;

#[component]
pub fn Terminal() -> Element {
    let mut command = use_signal(String::new);
    let mut output = use_signal(String::new);
    let settings = use_context::<Signal<AdbStudioSettings>>();

    rsx! {
        div {
            h2 { "Command Console" }
            div { style: "display: flex; gap: 8px; margin-bottom: 12px;",
                input { 
                    r#type: "text", 
                    placeholder: "Enter ADB command",
                    oninput: move |event| command.set(event.value().clone()),
                    style: "flex: 1;",
                }
                button { 
                    class: "btn",
                    onclick: move |_| {
                        spawn(async move {
                            let cmd = command.read().clone();
                            let settings_clone = settings.read().clone();
                            match adb_shell(&settings_clone, &cmd).await {
                                Ok(result) => output.set(result),
                                Err(e) => output.set(e),
                            }
                        });
                    },
                    i { class: "fas fa-play" }
                    "Execute" 
                }
            }
            div {
                class: "terminal-output",
                pre { {output} }
            }
        }
    }
}
