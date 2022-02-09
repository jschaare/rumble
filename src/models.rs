#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerItem {
    #[serde(rename = "itemID")]
    item_id: i32,
    can_use: bool,
    consumable: bool,
    count: i32,
    display_name: String,
    price: i32,
    raw_description: String,
    raw_display_name: String,
    slot: i32,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerScore {
    assists: i32,
    creep_score: i32,
    deaths: i32,
    kills: i32,
    ward_score: i32,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GameStats {
    game_mode: String,
    game_time: f32,
    map_name: String,
    map_number: i32,
    map_terrain: String,
}

/*
#[derive(Serialize, Deserialize)]
#[serde(tag = "EventName", rename_all = "PascalCase")]
enum GameEvent {
    #[serde(rename = "MinionsSpawning")]
    MinionsSpawning {
        #[serde(rename = "EventID")]
        event_id: i32,
        #[serde(rename = "EventTime")]
        event_time: f32,
    },
    #[serde(rename = "ChampionKill")]
    ChampionKill {
        event_id: i32,
        event_time: f32,
        killer_name: String,
        victim_name: String,
        assisters: Vec<String>,
    },
}
*/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_item() {
        let item_str = r#"{
            "canUse": false,
            "consumable": false,
            "count": 1,
            "displayName": "Doran's Blade",
            "itemID": 1055,
            "price": 450,
            "rawDescription": "GeneratedTip_Item_1055_Description",
            "rawDisplayName": "Item_1055_Name",
            "slot": 0
        }"#;
        let p: PlayerItem = serde_json::from_str(item_str).unwrap();
        assert_eq!(p.item_id, 1055);
        assert_eq!(p.display_name, "Doran's Blade");
        assert_eq!(p.can_use, false);
    }

    #[test]
    fn deserialize_item_list() {
        let item_list_str = r#"[
            {
                "canUse": false,
                "consumable": false,
                "count": 1,
                "displayName": "Doran's Blade",
                "itemID": 1055,
                "price": 450,
                "rawDescription": "GeneratedTip_Item_1055_Description",
                "rawDisplayName": "Item_1055_Name",
                "slot": 0
            },
            {
                "canUse": false,
                "consumable": false,
                "count": 1,
                "displayName": "Berserker's Greaves",
                "itemID": 3006,
                "price": 500,
                "rawDescription": "GeneratedTip_Item_3006_Description",
                "rawDisplayName": "Item_3006_Name",
                "slot": 1
            },
            {
                "canUse": false,
                "consumable": false,
                "count": 1,
                "displayName": "Zeal",
                "itemID": 3086,
                "price": 150,
                "rawDescription": "GeneratedTip_Item_3086_Description",
                "rawDisplayName": "Item_3086_Name",
                "slot": 2
            },
            {
                "canUse": false,
                "consumable": false,
                "count": 1,
                "displayName": "Immortal Shieldbow",
                "itemID": 6673,
                "price": 600,
                "rawDescription": "GeneratedTip_Item_6673_Description",
                "rawDisplayName": "Item_6673_Name",
                "slot": 3
            },
            {
                "canUse": true,
                "consumable": false,
                "count": 1,
                "displayName": "Stealth Ward",
                "itemID": 3340,
                "price": 0,
                "rawDescription": "GeneratedTip_Item_3340_Description",
                "rawDisplayName": "Item_3340_Name",
                "slot": 6
            }
        ]"#;
        let p: Vec<PlayerItem> = serde_json::from_str(item_list_str).unwrap();
        assert_eq!(p[0].item_id, 1055);
        assert_eq!(p[1].item_id, 3006);
        assert_eq!(p[2].item_id, 3086);
        assert_eq!(p[3].item_id, 6673);
        assert_eq!(p[4].item_id, 3340);
    }

    #[test]
    fn deserialize_score() {
        let score_str = r#"{
            "assists": 10,
            "creepScore": 100,
            "deaths": 0,
            "kills": 1,
            "wardScore": 0
        }"#;
        let s: PlayerScore = serde_json::from_str(score_str).unwrap();
        assert_eq!(s.assists, 10);
        assert_eq!(s.creep_score, 100);
        assert_eq!(s.deaths, 0);
        assert_eq!(s.kills, 1);
        assert_eq!(s.ward_score, 0);
    }

    #[test]
    fn deserialize_gamestats() {
        let stats_str = r#"{
            "gameMode": "PRACTICETOOL",
            "gameTime": 74.783203125,
            "mapName": "Map11",
            "mapNumber": 11,
            "mapTerrain": "Default"
        }"#;
        let s: GameStats = serde_json::from_str(stats_str).unwrap();
        assert_eq!(s.game_mode, "PRACTICETOOL");
        assert_eq!(s.game_time, 74.783203125);
        assert_eq!(s.map_name, "Map11");
        assert_eq!(s.map_number, 11);
        assert_eq!(s.map_terrain, "Default");
    }

    /*
    #[test]
    fn deserialize_event() {
        let event_str = r#"{
            "Assisters": [],
            "EventID": 2,
            "EventName": "ChampionKill",
            "EventTime": 154.3977508544922,
            "KillerName": "BRS",
            "VictimName": "Lux Bot"
        }"#;
        let e: GameEvent = serde_json::from_str(event_str).unwrap();
    }
    */
}
