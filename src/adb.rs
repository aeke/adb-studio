use std::process::Command;
use std::fs::File;
use std::io::Write;
use crate::settings::AdbStudioSettings;

fn get_adb_command(settings: &AdbStudioSettings) -> Command {
    Command::new(if settings.adb_path.is_empty() { "adb" } else { &settings.adb_path })
}

pub async fn adb_devices(settings: &AdbStudioSettings) -> Result<String, String> {
    let output = get_adb_command(settings).arg("devices").output();

    match output {
        Ok(output) => {
            if output.status.success() {
                Ok(String::from_utf8_lossy(&output.stdout).to_string())
            } else {
                Err(String::from_utf8_lossy(&output.stderr).to_string())
            }
        }
        Err(e) => Err(e.to_string()),
    }
}

pub async fn adb_reboot(settings: &AdbStudioSettings, serial: &str) -> Result<(), String> {
    let output = get_adb_command(settings)
        .arg("-s")
        .arg(serial)
        .arg("reboot")
        .output();

    match output {
        Ok(output) => {
            if output.status.success() {
                Ok(())
            } else {
                Err(String::from_utf8_lossy(&output.stderr).to_string())
            }
        }
        Err(e) => Err(e.to_string()),
    }
}

pub async fn adb_reboot_recovery(settings: &AdbStudioSettings, serial: &str) -> Result<(), String> {
    let output = get_adb_command(settings)
        .arg("-s")
        .arg(serial)
        .arg("reboot")
        .arg("recovery")
        .output();

    match output {
        Ok(output) => {
            if output.status.success() {
                Ok(())
            } else {
                Err(String::from_utf8_lossy(&output.stderr).to_string())
            }
        }
        Err(e) => Err(e.to_string()),
    }
}

pub async fn adb_reboot_bootloader(settings: &AdbStudioSettings, serial: &str) -> Result<(), String> {
    let output = get_adb_command(settings)
        .arg("-s")
        .arg(serial)
        .arg("reboot")
        .arg("bootloader")
        .output();

    match output {
        Ok(output) => {
            if output.status.success() {
                Ok(())
            } else {
                Err(String::from_utf8_lossy(&output.stderr).to_string())
            }
        }
        Err(e) => Err(e.to_string()),
    }
}

pub async fn adb_disconnect(settings: &AdbStudioSettings, serial: &str) -> Result<(), String> {
    let output = get_adb_command(settings).arg("disconnect").arg(serial).output();

    match output {
        Ok(output) => {
            if output.status.success() {
                Ok(())
            } else {
                Err(String::from_utf8_lossy(&output.stderr).to_string())
            }
        }
        Err(e) => Err(e.to_string()),
    }
}

pub async fn adb_shell(settings: &AdbStudioSettings, command: &str) -> Result<String, String> {
    let output = get_adb_command(settings).arg("shell").arg(command).output();

    match output {
        Ok(output) => {
            if output.status.success() {
                Ok(String::from_utf8_lossy(&output.stdout).to_string())
            } else {
                Err(String::from_utf8_lossy(&output.stderr).to_string())
            }
        }
        Err(e) => Err(e.to_string()),
    }
}

pub async fn adb_push(settings: &AdbStudioSettings, serial: &str, from: &str, to: &str) -> Result<String, String> {
    let output = get_adb_command(settings)
        .arg("-s")
        .arg(serial)
        .arg("push")
        .arg(from)
        .arg(to)
        .output();

    match output {
        Ok(output) => {
            if output.status.success() {
                Ok(String::from_utf8_lossy(&output.stdout).to_string())
            } else {
                Err(String::from_utf8_lossy(&output.stderr).to_string())
            }
        }
        Err(e) => Err(e.to_string()),
    }
}

pub async fn adb_pull(settings: &AdbStudioSettings, serial: &str, from: &str, to: &str) -> Result<String, String> {
    let output = get_adb_command(settings)
        .arg("-s")
        .arg(serial)
        .arg("pull")
        .arg(from)
        .arg(to)
        .output();

    match output {
        Ok(output) => {
            if output.status.success() {
                Ok(String::from_utf8_lossy(&output.stdout).to_string())
            } else {
                Err(String::from_utf8_lossy(&output.stderr).to_string())
            }
        }
        Err(e) => Err(e.to_string()),
    }
}

pub async fn adb_install(settings: &AdbStudioSettings, serial: &str, apk_path: &str) -> Result<String, String> {
    let output = get_adb_command(settings)
        .arg("-s")
        .arg(serial)
        .arg("install")
        .arg(apk_path)
        .output();

    match output {
        Ok(output) => {
            if output.status.success() {
                Ok(String::from_utf8_lossy(&output.stdout).to_string())
            } else {
                Err(String::from_utf8_lossy(&output.stderr).to_string())
            }
        }
        Err(e) => Err(e.to_string()),
    }
}

#[allow(dead_code)]
pub async fn adb_uninstall(settings: &AdbStudioSettings, serial: &str, package_name: &str) -> Result<String, String> {
    let output = get_adb_command(settings)
        .arg("-s")
        .arg(serial)
        .arg("uninstall")
        .arg(package_name)
        .output();

    match output {
        Ok(output) => {
            if output.status.success() {
                Ok(String::from_utf8_lossy(&output.stdout).to_string())
            } else {
                Err(String::from_utf8_lossy(&output.stderr).to_string())
            }
        }
        Err(e) => Err(e.to_string()),
    }
}

pub async fn adb_list_packages(settings: &AdbStudioSettings, serial: &str) -> Result<String, String> {
    let output = get_adb_command(settings)
        .arg("-s")
        .arg(serial)
        .arg("shell")
        .arg("pm list packages")
        .output();

    match output {
        Ok(output) => {
            if output.status.success() {
                Ok(String::from_utf8_lossy(&output.stdout).to_string())
            } else {
                Err(String::from_utf8_lossy(&output.stderr).to_string())
            }
        }
        Err(e) => Err(e.to_string()),
    }
}

pub async fn adb_screenshot(settings: &AdbStudioSettings, serial: &str, path: &str) -> Result<String, String> {
    let output = get_adb_command(settings)
        .arg("-s")
        .arg(serial)
        .arg("exec-out")
        .arg("screencap -p")
        .output();

    match output {
        Ok(output) => {
            if output.status.success() {
                let mut file = File::create(path).map_err(|e| e.to_string())?;
                file.write_all(&output.stdout).map_err(|e| e.to_string())?;
                Ok(format!("Screenshot saved to {}", path))
            } else {
                Err(String::from_utf8_lossy(&output.stderr).to_string())
            }
        }
        Err(e) => Err(e.to_string()),
    }
}

pub async fn adb_start_screenrecord(settings: &AdbStudioSettings, serial: &str, path: &str) -> Result<String, String> {
    let output = get_adb_command(settings)
        .arg("-s")
        .arg(serial)
        .arg("shell")
        .arg(format!("screenrecord {}", path))
        .spawn(); // Use spawn to run in background

    match output {
        Ok(_) => Ok("Screen recording started.".to_string()),
        Err(e) => Err(e.to_string()),
    }
}

pub async fn adb_stop_screenrecord(settings: &AdbStudioSettings, serial: &str) -> Result<String, String> {
    let output = get_adb_command(settings)
        .arg("-s")
        .arg(serial)
        .arg("shell")
        .arg("pkill -INT screenrecord")
        .output();

    match output {
        Ok(output) => {
            if output.status.success() {
                Ok(String::from_utf8_lossy(&output.stdout).to_string())
            } else {
                Err(String::from_utf8_lossy(&output.stderr).to_string())
            }
        }
        Err(e) => Err(e.to_string()),
    }
}
