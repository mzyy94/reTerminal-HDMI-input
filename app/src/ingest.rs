use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Clone)]
pub struct TwitchIngest {
    pub name: String,
    pub url_template: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Twitch {
    pub ingests: Vec<TwitchIngest>,
}

impl Twitch {
    pub async fn get_ingests() -> Option<Twitch> {
        let url = "https://ingest.twitch.tv/ingests";
        reqwest::get(url).await.ok()?.json().await.ok()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Service {
    YouTubeLive,
    Twitch,
    Custom,
}

impl Service {
    pub const ALL: [Service; 3] = [Service::YouTubeLive, Service::Twitch, Service::Custom];
}

impl Default for Service {
    fn default() -> Service {
        Service::Custom
    }
}

#[derive(Debug, Clone)]
pub enum IngestError {
    InvalidSetting,
}

impl Service {
    pub async fn get_ingest_url() -> Result<String, IngestError> {
        let service = crate::SETTINGS.read().unwrap().ingest_service;
        if let Some(service) = service {
            Ok(match service {
                Service::YouTubeLive => "rtmp://a.rtmp.youtube.com/live2/{stream_key}".to_string(),
                Service::Twitch => Twitch::get_ingests()
                    .await
                    .unwrap()
                    .ingests
                    .first()
                    .unwrap()
                    .url_template
                    .clone(),
                Service::Custom => crate::SETTINGS.read().unwrap().rtmp_url.clone(),
            })
        } else {
            Err(IngestError::InvalidSetting)
        }
    }
}

impl std::fmt::Display for Service {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Service::YouTubeLive => "YouTube Live",
                Service::Twitch => "Twitch",
                Service::Custom => "Custom URL",
            }
        )
    }
}

impl std::str::FromStr for Service {
    type Err = serde_json::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_json::from_value(serde_json::Value::String(s.to_string()))
    }
}
