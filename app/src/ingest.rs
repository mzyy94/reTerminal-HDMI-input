use serde::Deserialize;

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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "YouTube Live" => Ok(Service::YouTubeLive),
            "Twitch" => Ok(Service::Twitch),
            "Custom URL" => Ok(Service::Custom),
            _ => Err("Invalid string".to_string()),
        }
    }
}
