use reqwest::Response;

pub struct ResponseInfo {
    pub response: Response,
    pub retries: u8,
}
