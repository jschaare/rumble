use crate::live::models;
use reqwest::{Client, Method, RequestBuilder};

use crate::live::config::LiveClientConfig;

pub struct LiveClient {
    config: LiveClientConfig,
    client: Client,
}

impl LiveClient {
    pub fn new(config: impl Into<LiveClientConfig>) -> Self {
        Self {
            config: config.into(),
            client: Client::builder().build().unwrap(), //TODO riot pem
        }
    }

    pub fn request(&self, method: Method, path: &str) -> RequestBuilder {
        self.client
            .request(method, format!("{}{}", self.config.base_url, path))
    }
}
