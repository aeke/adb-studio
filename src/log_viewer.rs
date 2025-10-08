use dioxus::prelude::*;
use tokio::io::{BufReader, AsyncBufReadExt};
use tokio::process::Command;
use tokio::sync::mpsc;
use crate::settings::AdbStudioSettings;

#[component]
pub fn LogViewer() -> Element {
    let mut logs = use_signal(String::new);
    let mut is_logging = use_signal(|| false);
    let settings = use_context::<Signal<AdbStudioSettings>>();

    use_effect(move || {
        if *is_logging.read() {
            let (tx, mut rx_stream) = mpsc::channel(100);
            let settings_clone = settings.read().clone();
            spawn(async move {
                let mut cmd = Command::new(if settings_clone.adb_path.is_empty() { "adb" } else { &settings_clone.adb_path });
                cmd.arg("logcat");
                let mut child = cmd.stdout(std::process::Stdio::piped()).spawn().expect("Failed to spawn adb logcat");
                let stdout = child.stdout.take().expect("Failed to get stdout");
                let mut reader = BufReader::new(stdout).lines();

                while let Ok(Some(line)) = reader.next_line().await {
                    if tx.send(line).await.is_err() {
                        break;
                    }
                }
                child.wait().await.expect("Logcat command failed");
            });

            spawn(async move {
                while let Some(line) = rx_stream.recv().await {
                    logs.with_mut(|l| {
                        l.push_str(&line);
                        l.push('\n');
                    });
                }
            });
        }
    });

    rsx! {
        div {
            h2 { "Real-time Logs" }
            div {
                button {
                    class: "btn",
                    onclick: move |_| {
                        let current_logging_state = *is_logging.read();
                        is_logging.set(!current_logging_state);
                    },
                    i { class: if *is_logging.read() { "fas fa-stop" } else { "fas fa-play" } }
                    if *is_logging.read() { "Stop Logcat" } else { "Start Logcat" }
                }
            }
            div {
                class: "log-output",
                pre { {logs} }
            }
        }
    }
}
