use reqwest::Method;

use crate::live::client::LiveClient;
use crate::live::error::{LiveClientError, LiveClientResult};
use crate::live::models::*;

impl LiveClient {
    pub fn player_api(&self) -> PlayerApi {
        PlayerApi { client: self }
    }

    pub fn game_api(&self) -> GameApi {
        GameApi { client: self }
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

    pub async fn get_player_items(&self, player_name: &str) -> LiveClientResult<Vec<PlayerItem>> {
        let request = self.client.request(
            Method::GET,
            &format!("/liveclientdata/playeritems?summonerName={}", player_name),
        );
        let response = self.client.execute::<Vec<PlayerItem>>(request).await;
        response
    }

    pub async fn get_player_list(&self) -> LiveClientResult<Vec<Player>> {
        let request = self.client.request(
            Method::GET,
            &format!("/liveclientdata/playerlist"),
        );
        let response = self.client.execute::<Vec<Player>>(request).await;
        response
    }

    pub async fn get_active_player_abilities(&self) -> LiveClientResult<ActivePlayerAbilities> {
        let request = self.client.request(
            Method::GET,
            &format!("/liveclientdata/activeplayerabilities"),
        );
        let response = self.client.execute::<ActivePlayerAbilities>(request).await;
        response
    }

    pub async fn get_active_player_runes(&self) -> LiveClientResult<ActivePlayerRunes> {
        let request = self.client.request(
            Method::GET,
            &format!("/liveclientdata/activeplayerrunes"),
        );
        let response = self.client.execute::<ActivePlayerRunes>(request).await;
        response
    }

    pub async fn get_active_player(&self) -> LiveClientResult<ActivePlayer> {
        let request = self.client.request(
            Method::GET,
            &format!("/liveclientdata/activeplayer"),
        );
        let response = self.client.execute::<ActivePlayer>(request).await;
        response
    }

    pub async fn get_active_player_name(&self) -> LiveClientResult<String> {
        let request = self.client.request(
            Method::GET,
            &format!("/liveclientdata/activeplayername"),
        );
        let response = self.client.execute::<String>(request).await;
        response
    }
}

pub struct GameApi<'a> {
    client: &'a LiveClient,
}

impl<'a> GameApi<'a> {
    pub async fn get_game_stats(&self) -> LiveClientResult<GameStats> {
        let request = self.client.request(
            Method::GET,
            &format!("/liveclientdata/gamestats"),
        );
        let response = self.client.execute::<GameStats>(request).await;
        response
    }

    pub async fn get_game_events(&self) -> LiveClientResult<GameEventAll> {
        let request = self.client.request(
            Method::GET,
            &format!("/liveclientdata/eventdata"),
        );
        let response = self.client.execute::<GameEventAll>(request).await;
        response
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::live::config::LiveClientConfig;

    #[tokio::test]
    async fn get_active_player() {
        let lcu = LiveClient::new(LiveClientConfig::new());
        let player: ActivePlayer = lcu.player_api().get_active_player().await.unwrap();
        println!("{}", player.summoner_name);
    }

    #[tokio::test]
    async fn get_active_player_runes() {
        let lcu = LiveClient::new(LiveClientConfig::new());
        let player: ActivePlayerRunes = lcu.player_api().get_active_player_runes().await.unwrap();
    }

    #[tokio::test]
    async fn get_active_player_abilities() {
        let lcu = LiveClient::new(LiveClientConfig::new());
        let player: ActivePlayerAbilities = lcu.player_api().get_active_player_abilities().await.unwrap();
    }

    #[tokio::test]
    async fn get_player_list() {
        let lcu = LiveClient::new(LiveClientConfig::new());
        let players: Vec<Player> = lcu.player_api().get_player_list().await.unwrap();
        println!("{}", players.len());
    }

    #[tokio::test]
    async fn get_game_events() {
        let lcu = LiveClient::new(LiveClientConfig::new());
        let allevents: GameEventAll = lcu.game_api().get_game_events().await.unwrap();
        println!("{}", allevents.events.len());
        println!("{:?}", allevents.events);
    }

    #[tokio::test]
    async fn get_game_stats() {
        let lcu = LiveClient::new(LiveClientConfig::new());
        let stats: GameStats = lcu.game_api().get_game_stats().await.unwrap();
        println!("{:?}", stats);
    }
}