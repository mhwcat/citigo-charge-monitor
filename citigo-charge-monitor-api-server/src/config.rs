use std::net::SocketAddr;

pub struct Config {
    pub database_url: String,
    pub redis_url: String,
    pub api_base_addr: SocketAddr,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            database_url: std::env::var("CITIGO_DATABASE_URL")
                .expect("Missing CITIGO_DATABASE_URL env var"),
            redis_url: std::env::var("CITIGO_REDIS_URL").expect("Missing CITIGO_REDIS_URL env var"),
            api_base_addr: std::env::var("CITIGO_API_BASE_ADDR")
                .expect("Missing CITIGO_API_BASE_ADDR env var")
                .parse()
                .expect("Failed parsing API base addr"),
        }
    }
}
