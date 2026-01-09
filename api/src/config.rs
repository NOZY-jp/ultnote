use std::env;

pub struct Config {
    pub qdrant_url: String,
    pub embedder_url: String,
    pub host: String,
    pub port: u16,
    pub cf_team_domain: Option<String>,
    pub cf_policy_aud: Option<String>,
}

impl Config {
    pub fn from_env() -> Self {
        Self {
            qdrant_url: env::var("QDRANT_URL").unwrap_or_else(|_| "http://localhost:6334".into()),
            embedder_url: env::var("EMBEDDER_URL")
                .unwrap_or_else(|_| "http://localhost:8000".into()),
            host: env::var("HOST").unwrap_or_else(|_| "0.0.0.0".into()),
            port: env::var("PORT")
                .ok()
                .and_then(|p| p.parse().ok())
                .unwrap_or(8080),
            cf_team_domain: env::var("CF_TEAM_DOMAIN").ok(),
            cf_policy_aud: env::var("CF_POLICY_AUD").ok(),
        }
    }
}
