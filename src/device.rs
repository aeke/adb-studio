use serde::Deserialize;

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct Device {
    pub serial: String,
    pub status: String,
    pub model: String,
}

pub fn parse_devices(output: &str) -> Vec<Device> {
    let mut devices = Vec::new();
    for line in output.lines().skip(1) {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() >= 2 {
            devices.push(Device {
                serial: parts[0].to_string(),
                status: parts[1].to_string(),
                model: "".to_string(), // Model detection requires more commands
            });
        }
    }
    devices
}
