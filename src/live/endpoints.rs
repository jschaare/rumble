use reqwest::Method;

use crate::live::client::LiveClient;
use crate::live::error::{LiveClientError, LiveClientResult};
use crate::live::models::*;

impl LiveClient {
    pub fn player_api(&self) -> PlayerApi {
        PlayerApi { client: self }
    }
}

pub struct PlayerApi<'a> {
    client: &'a LiveClient,
}

impl<'a> PlayerApi<'a> {
    pub async fn get_player_score(&self, player_name: &str) -> LiveClientResult<PlayerScore> {
        let request = self.client.request(
            Method::GET,
            &format!("/liveclientdata/playerscores?summonerName={}", player_name),
        );
        let response = self.client.execute::<PlayerScore>(request).await;
        response
    }
}
