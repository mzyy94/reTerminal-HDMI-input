use crate::ingest::Service;
use std::env;

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
}