use reqwest::{Client, Method, RequestBuilder};

use crate::live::{
    config::LiveClientConfig,
    error::{LiveClientError, LiveClientResult},
    response::ResponseInfo,
};

use crate::live::cert;

pub struct LiveClient {
    config: LiveClientConfig,
    client: Client,
}

impl LiveClient {
    pub fn new(config: impl Into<LiveClientConfig>) -> Self {
        Self {
            config: config.into(),
            client: Client::builder()
                .add_root_certificate(cert::get_certificate())
                .build()
                .unwrap(), //TODO riot pem
        }
    }

    pub fn request(&self, method: Method, path: &str) -> RequestBuilder {
        self.client
            .request(method, format!("{}{}", self.config.base_url, path))
    }

    pub async fn execute<'a, T: serde::de::DeserializeOwned + 'a>(
        &self,
        request: RequestBuilder,
    ) -> LiveClientResult<T> {
        let rinfo = self.execute_raw(request).await?;
        let status = rinfo.response.status();
        let value = rinfo.response.json::<T>().await;
        value.map_err(|e| LiveClientError::new(e, rinfo.retries, None, Some(status)))
    }

    pub async fn execute_raw(&self, request: RequestBuilder) -> LiveClientResult<ResponseInfo> {
        let mut retries: u8 = 0;
        loop {
            let request_clone = request
                .try_clone()
                .expect("Failed to clone request.")
                .send();
            let response = request_clone
                .await
                .map_err(|e| LiveClientError::new(e, retries, None, None))?;

            let status = response.status();
            if status.is_success() {
                break Ok(ResponseInfo { response, retries });
            } else if retries > self.config.retries {
                let err = response.error_for_status_ref().err().unwrap();
                break Err(LiveClientError::new(err, retries, None, None));
            }
            retries += 1;
        }
    }
}
