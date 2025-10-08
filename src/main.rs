#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod adb;
mod config;
mod device;
mod fileops;
mod theme;
mod terminal;
mod ui;
mod app_manager;
mod media;
mod settings;
mod log_viewer;
mod app;

use app::App;
use dioxus::prelude::*;

#[cfg(target_os = "macos")]
fn fix_path() {
    use std::env;
    
    let current_path = env::var("PATH").unwrap_or_default();
    let home = env::var("HOME").unwrap_or_default();
    
    let mut paths_to_add = Vec::new();
    
    if !current_path.contains("/usr/local/bin") {
        paths_to_add.push("/usr/local/bin");
    }
    if !current_path.contains("/opt/homebrew/bin") {
        paths_to_add.push("/opt/homebrew/bin");
    }
    
    let android_sdk_path = format!("{}/Library/Android/sdk/platform-tools", home);
    if !current_path.contains(&android_sdk_path) {
        paths_to_add.push(&android_sdk_path);
    }
    
    if !paths_to_add.is_empty() {
        let new_path = format!("{}:{}", paths_to_add.join(":"), current_path);
        env::set_var("PATH", new_path);
    }
}

#[cfg(not(target_os = "macos"))]
fn fix_path() {}

fn main() {
    fix_path();
    
    dioxus_logger::init(tracing::Level::INFO).expect("failed to init logger");
    
    let icon_bytes = include_bytes!("../assets/appicon.png");
    let icon_image = image::load_from_memory(icon_bytes).expect("Failed to load icon");
    let icon_rgba = icon_image.to_rgba8();
    let (width, height) = icon_rgba.dimensions();
    let icon = dioxus::desktop::tao::window::Icon::from_rgba(icon_rgba.into_raw(), width, height)
        .expect("Failed to create icon");
    
    LaunchBuilder::desktop()
        .with_cfg(dioxus::desktop::Config::new()
            .with_window(dioxus::desktop::WindowBuilder::new()
                .with_title("ADB Studio")
                .with_window_icon(Some(icon))
            )
            .with_custom_head(
            r#"<link rel="stylesheet" href="https://cdnjs.cloudflare.com/ajax/libs/font-awesome/6.5.1/css/all.min.css"/><style>
:root{--primary-bg:#0d1117;--secondary-bg:#161b22;--tertiary-bg:#21262d;--card-bg:#161b22;--primary-text:#e6edf3;--secondary-text:#8b949e;--accent-color:#2f81f7;--accent-hover:#1f6feb;--accent-light:rgba(47,129,247,0.15);--success-color:#3fb950;--warning-color:#d29922;--error-color:#f85149;--border-color:#30363d;--shadow:0 2px 8px rgba(0,0,0,0.4);--shadow-lg:0 8px 24px rgba(0,0,0,0.5);--font-family:-apple-system,BlinkMacSystemFont,'Segoe UI',Roboto,sans-serif;--transition:all 0.2s ease}*{box-sizing:border-box;margin:0;padding:0}body,html{font-family:var(--font-family);background:var(--primary-bg);color:var(--primary-text);overflow:hidden;font-size:14px;line-height:1.5;width:100%;height:100%}.light-theme{--primary-bg:#f6f8fa;--secondary-bg:#ffffff;--tertiary-bg:#f6f8fa;--card-bg:#ffffff;--primary-text:#1f2328;--secondary-text:#1f2328;--accent-color:#0969da;--accent-hover:#0550ae;--accent-light:rgba(9,105,218,0.1);--success-color:#1a7f37;--warning-color:#9a6700;--error-color:#cf222e;--border-color:#d0d7de;--shadow:0 2px 8px rgba(0,0,0,0.08);--shadow-lg:0 8px 24px rgba(0,0,0,0.12)}.app{display:flex;height:100vh;width:100vw}.light-theme.app{background:var(--primary-bg)}.sidebar{width:220px;background:var(--secondary-bg);padding:20px 12px;display:flex;flex-direction:column;border-right:1px solid var(--border-color);overflow-y:auto}.sidebar h1{font-size:1.3rem;font-weight:600;margin:0 0 24px 12px;color:var(--accent-color)}.sidebar nav{display:flex;flex-direction:column;gap:4px}.sidebar nav button{display:flex;align-items:center;gap:12px;width:100%;padding:10px 14px;background:transparent;border:none;color:var(--secondary-text);text-align:left;font-size:0.9rem;font-weight:500;cursor:pointer;border-radius:6px;transition:var(--transition)}.sidebar nav button i{width:18px;text-align:center;font-size:1rem}.alert-message{position:fixed;top:20px;right:20px;padding:12px 20px;background:var(--warning-color);color:#fff;border-radius:8px;font-size:0.875rem;font-weight:500;box-shadow:var(--shadow-lg);z-index:1000;animation:slideIn 0.3s ease}@keyframes slideIn{from{transform:translateX(400px);opacity:0}to{transform:translateX(0);opacity:1}}select.btn{cursor:pointer;font-family:var(--font-family)}select.btn option{background:var(--card-bg);color:var(--primary-text);padding:10px;font-size:0.875rem;font-family:var(--font-family)}.sidebar nav button:hover{background:var(--tertiary-bg);color:var(--primary-text)}.main-content{flex:1;display:flex;flex-direction:column;overflow:hidden}.top-toolbar{display:flex;align-items:center;gap:10px;padding:12px 20px;background:var(--secondary-bg);border-bottom:1px solid var(--border-color)}.top-toolbar button{padding:6px 14px;background:var(--tertiary-bg);border:1px solid var(--border-color);color:var(--primary-text);border-radius:6px;cursor:pointer;font-size:0.875rem;font-weight:500;transition:var(--transition);display:flex;align-items:center;gap:6px}.top-toolbar button:hover{background:var(--accent-color);border-color:var(--accent-color)}.top-toolbar span{margin-left:auto;font-size:0.875rem;color:var(--secondary-text);display:flex;align-items:center;gap:6px}.content-area{padding:20px;overflow-y:auto;background:var(--primary-bg)}.content-area::-webkit-scrollbar{width:8px}.content-area::-webkit-scrollbar-track{background:transparent}.content-area::-webkit-scrollbar-thumb{background:var(--tertiary-bg);border-radius:4px}.content-area::-webkit-scrollbar-thumb:hover{background:var(--border-color)}.content-area h2{margin:0 0 20px 0;font-size:1.5rem;font-weight:600;color:var(--primary-text)}.btn{padding:8px 16px;background:var(--accent-color);border:none;color:#fff;border-radius:6px;cursor:pointer;font-size:0.875rem;font-weight:500;transition:var(--transition);display:inline-flex;align-items:center;gap:8px;margin-right:8px;margin-bottom:8px}.btn:hover{background:var(--accent-hover);transform:translateY(-1px)}.btn:active{transform:translateY(0)}.btn i{font-size:0.875rem}.btn:disabled{opacity:0.5;cursor:not-allowed;transform:none!important}.btn-secondary{background:var(--tertiary-bg);color:var(--primary-text);border:1px solid var(--border-color)}.btn-secondary:hover{background:var(--border-color)}.btn-secondary:disabled{opacity:0.5;cursor:not-allowed}.dashboard-grid{display:grid;grid-template-columns:repeat(auto-fit,minmax(240px,1fr));gap:16px;margin-bottom:20px}.stat-card{background:var(--card-bg);border:1px solid var(--border-color);border-radius:8px;padding:20px;transition:var(--transition)}.stat-card:hover{border-color:var(--accent-color);transform:translateY(-2px);box-shadow:var(--shadow)}.stat-card h3{font-size:0.75rem;font-weight:600;color:var(--secondary-text);text-transform:uppercase;letter-spacing:0.5px;margin-bottom:8px}.stat-card .stat-value{font-size:1.75rem;font-weight:600;color:var(--primary-text);margin-bottom:4px}.stat-card .stat-label{font-size:0.875rem;color:var(--secondary-text)}.terminal-output,.log-output{background:var(--card-bg);border:1px solid var(--border-color);border-radius:6px;padding:12px;margin-top:12px;height:400px;overflow-y:auto;font-family:'SF Mono','Monaco','Courier New',monospace;font-size:0.8rem}.terminal-output::-webkit-scrollbar,.log-output::-webkit-scrollbar{width:8px}.terminal-output::-webkit-scrollbar-track,.log-output::-webkit-scrollbar-track{background:transparent}.terminal-output::-webkit-scrollbar-thumb,.log-output::-webkit-scrollbar-thumb{background:var(--tertiary-bg);border-radius:4px}.terminal-output pre,.log-output pre{white-space:pre-wrap;word-wrap:break-word;margin:0;color:var(--success-color);line-height:1.5}table{width:100%;border-collapse:separate;border-spacing:0;margin-top:16px;background:var(--card-bg);border:1px solid var(--border-color);border-radius:8px;overflow:hidden}th,td{padding:10px 12px;text-align:left;border-bottom:1px solid var(--border-color);font-size:0.875rem;color:var(--primary-text)}th{background:var(--tertiary-bg);font-weight:600;color:var(--secondary-text);text-transform:uppercase;font-size:0.75rem;letter-spacing:0.5px}tbody tr{transition:var(--transition);cursor:pointer}tbody tr:hover{background:var(--tertiary-bg)}tbody tr.selected{background:var(--accent-light)!important;border-left:3px solid var(--accent-color)}tbody tr:last-child td{border-bottom:none}td button{padding:6px 12px;margin-right:6px;background:var(--tertiary-bg);border:1px solid var(--border-color);color:var(--primary-text);border-radius:6px;cursor:pointer;font-size:0.8rem;font-weight:500;transition:var(--transition);display:inline-flex;align-items:center;gap:4px}td button:hover{background:var(--accent-color);border-color:var(--accent-color);color:#fff}td button i{font-size:0.75rem}input[type="text"],input[type="checkbox"]{padding:8px 12px;background:var(--tertiary-bg);border:1px solid var(--border-color);color:var(--primary-text);border-radius:6px;font-size:0.875rem;font-family:var(--font-family);width:100%;max-width:400px}input[type="text"]:focus{outline:none;border-color:var(--accent-color)}input[type="checkbox"]{width:auto;margin-left:8px}.form-group{margin-bottom:16px}.form-group label{display:block;margin-bottom:6px;font-size:0.875rem;font-weight:500;color:var(--primary-text)}ul{list-style:none;padding:0}ul li{padding:8px 12px;background:var(--card-bg);border:1px solid var(--border-color);border-radius:6px;margin-bottom:6px;font-size:0.875rem;color:var(--primary-text)}.package-item{padding:8px 12px;margin:4px 0;cursor:pointer;background:var(--tertiary-bg);border-radius:6px;transition:var(--transition)}.package-item:hover{background:var(--border-color)}.package-item.selected{background:var(--accent-color)!important;color:#fff}h3{font-size:1.1rem;font-weight:600;color:var(--primary-text);margin:16px 0 12px 0}.quick-actions{display:grid;grid-template-columns:repeat(auto-fit,minmax(200px,1fr));gap:12px;margin-top:20px}.quick-action-card{background:var(--card-bg);border:1px solid var(--border-color);border-radius:8px;padding:16px;cursor:pointer;transition:var(--transition);text-align:center}.quick-action-card:hover{border-color:var(--accent-color);transform:translateY(-2px)}.quick-action-card i{font-size:2rem;color:var(--accent-color);margin-bottom:8px}.quick-action-card h4{font-size:0.875rem;font-weight:600;color:var(--primary-text);margin:0}.device-info{background:var(--card-bg);border:1px solid var(--border-color);border-radius:8px;padding:16px;margin-top:20px}.device-info h3{font-size:1rem;font-weight:600;margin-bottom:12px;color:var(--primary-text)}.device-info-item{display:flex;justify-content:space-between;padding:8px 0;border-bottom:1px solid var(--border-color);font-size:0.875rem}.device-info-item:last-child{border-bottom:none}.device-info-item .label{color:var(--secondary-text)}.device-info-item .value{color:var(--primary-text);font-weight:500}.dropdown-item:hover{background:var(--tertiary-bg)}
</style>"#.to_string(),
        ))
        .launch(App);
}
