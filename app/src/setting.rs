use crate::ingest::Service;
use serde::{Deserialize, Serialize};

use std::env;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::PathBuf;

#[derive(Deserialize, Serialize, Default, Debug)]
pub struct BroadcastSetting {
    pub ingest_service: Option<Service>,
    pub custom_url: String,
    pub stream_key: String,
}

#[derive(Deserialize, Serialize, Default, Debug)]
pub struct DeviceSetting {
    pub hdmi_device: Option<String>,
    pub camera_device: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MicrophoneMode {
    Normal,
    ForceStereo,
}

impl std::str::FromStr for MicrophoneMode {
    type Err = serde_json::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_json::from_value(serde_json::Value::String(s.to_string()))
    }
}

#[derive(Deserialize, Serialize, Default, Debug)]
pub struct MediaSetting {
    pub mic_mode: Option<MicrophoneMode>,
}

#[derive(Deserialize, Serialize, Default, Debug)]
pub struct Settings {
    pub broadcast: BroadcastSetting,
    pub device: DeviceSetting,
    pub media: MediaSetting,
}

impl Settings {
    fn config_file() -> Result<PathBuf, std::io::Error> {
        env::var_os("XDG_CONFIG_HOME")
            .map(PathBuf::from)
            .and_then(|path| if path.is_absolute() { Some(path) } else { None })
            .or_else(|| {
                env::var_os("HOME")
                    .and_then(|var| if var.is_empty() { None } else { Some(var) })
                    .map(PathBuf::from)
                    .map(|path| path.join(".config"))
            })
            .map(|path| path.join("broadcast-terminal.toml"))
            .ok_or(std::io::ErrorKind::NotFound.into())
    }

    pub fn new() -> Self {
        let setting: Self = Self::config_file()
            .and_then(std::fs::read_to_string)
            .ok()
            .and_then(|s| toml::from_str(&s).ok())
            .unwrap_or_default();

        println!("Load setting: {:?}", &setting);

        let custom_url = env::var("RTMP_URL").unwrap_or(setting.broadcast.custom_url);
        let stream_key = env::var("STREAM_KEY").unwrap_or(setting.broadcast.stream_key);
        let hdmi_device = env::var("HDMI_DEVICE").ok().or(setting.device.hdmi_device);
        let camera_device = env::var("CAMERA_DEVICE")
            .ok()
            .or(setting.device.camera_device);
        let ingest_service = env::var("INGEST_SERVICE")
            .ok()
            .and_then(|service| service.parse().ok())
            .or(setting.broadcast.ingest_service);

        let mic_mode = env::var("MIC_MODE")
            .ok()
            .and_then(|mode| mode.parse().ok())
            .or(setting.media.mic_mode);

        Settings {
            broadcast: BroadcastSetting {
                ingest_service,
                custom_url,
                stream_key,
            },
            device: DeviceSetting {
                hdmi_device,
                camera_device,
            },
            media: MediaSetting { mic_mode },
        }
    }

    pub fn save(&self) -> Result<(), std::io::Error> {
        let config_file = Self::config_file()?;

        OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(config_file)
            .and_then(|mut file| {
                let toml = toml::to_string(self).unwrap();
                write!(file, "{}", toml)
            })
    }
}
