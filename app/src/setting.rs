use crate::ingest::Service;
use serde::{Deserialize, Serialize};

use std::env;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::PathBuf;

#[derive(Deserialize, Serialize)]
pub struct Settings {
    pub rtmp_url: String,
    pub stream_key: String,
    pub hdmi_device: Option<String>,
    pub camera_device: Option<String>,
    pub ingest_service: Option<Service>,
}

impl Settings {
    pub fn new() -> Self {
        let rtmp_url = env::var("RTMP_URL").unwrap_or_default();
        let stream_key = env::var("STREAM_KEY").unwrap_or_default();
        let hdmi_device = env::var("HDMI_DEVICE").ok();
        let camera_device = env::var("CAMERA_DEVICE").ok();
        let ingest_service = env::var("INGEST_SERVICE")
            .ok()
            .and_then(|service| service.parse().ok());

        Settings {
            rtmp_url,
            stream_key,
            hdmi_device,
            camera_device,
            ingest_service,
        }
    }

    pub fn save(&self) -> Result<(), std::io::Error> {
        let config_dir = env::var_os("XDG_CONFIG_HOME")
            .map(PathBuf::from)
            .and_then(|path| if path.is_absolute() { Some(path) } else { None })
            .or_else(|| {
                env::var_os("HOME")
                    .and_then(|var| if var.is_empty() { None } else { Some(var) })
                    .map(PathBuf::from)
                    .map(|path| path.join(".config"))
            });

        let config_dir = if let Some(config_dir) = config_dir {
            config_dir
        } else {
            return Err(std::io::ErrorKind::NotFound.into());
        };

        OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(config_dir.join("broadcast-terminal.toml"))
            .and_then(|mut file| {
                let toml = toml::to_string(self).unwrap();
                write!(file, "{}", toml)
            })
    }
}
