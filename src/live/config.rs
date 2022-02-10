pub struct LiveClientConfig {
    pub base_url: String,
    pub retries: u8,
}

impl LiveClientConfig {
    pub const DEFAULT_BASE_URL: &'static str = "https://127.0.0.1:2999";
    pub const DEFAULT_RETRIES: u8 = 3;

    pub fn new() -> Self {
        Self {
            base_url: Self::DEFAULT_BASE_URL.into(),
            retries: Self::DEFAULT_RETRIES,
        }
    }
}
