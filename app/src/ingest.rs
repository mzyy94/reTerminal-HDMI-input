use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Ingest {
    pub name: String,
    pub url_template: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Twitch {
    pub ingests: Vec<Ingest>,
}

impl Twitch {
    pub async fn get_ingests() -> Option<Twitch> {
        let url = "https://ingest.twitch.tv/ingests";
        reqwest::get(url).await.ok()?.json().await.ok()
    }
}
