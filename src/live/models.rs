use std::collections::HashMap;

use serde::{de, Deserialize, Deserializer};

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
pub struct Player {
    champion_name: String,
    is_bot: bool,
    is_dead: bool,
    items: Vec<PlayerItem>,
    level: i32,
    position: String,
    raw_champion_name: String,
    respawn_timer: f64,
    runes: MainRunes,
    scores: PlayerScore,
    #[serde(rename = "skinID")]
    skin_id: i64,
    summoner_name: String,
    summoner_spells: SummonerSpells,
    team: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ActivePlayer {
    abilities: ActivePlayerAbilities,
    champion_stats: ChampionStats,
    current_gold: f64,
    full_runes: ActivePlayerRunes,
    level: i32,
    summoner_name: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ChampionStats {
    ability_haste: Option<f64>,
    ability_power: Option<f64>,
    armor: f64,
    armor_penetration_flat: f64,
    armor_penetration_percent: f64,
    attack_damage: f64,
    attack_range: f64,
    attack_speed: f64,
    bonus_armor_penetration_percent: f64,
    bonus_magic_penetration_percent: f64,
    cooldown_reduction: Option<f64>,
    crit_chance: Option<f64>,
    crit_damage: Option<f64>,
    current_health: f64,
    health_regen_rate: f64,
    life_steal: Option<f64>,
    magic_lethality: Option<f64>,
    magic_penetration_flat: Option<f64>,
    magic_penetration_percent: Option<f64>,
    magic_resist: Option<f64>,
    max_health: f64,
    move_speed: f64,
    physical_lethality: Option<f64>,
    resource_max: f64,
    resource_regen_rate: f64,
    resource_type: String,
    resource_value: f64,
    spell_vamp: Option<f64>,
    tenacity: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Ability {
    ability_level: Option<i32>,
    display_name: String,
    id: String,
    raw_description: String,
    raw_display_name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ActivePlayerAbilities {
    #[serde(flatten)]
    abilities: HashMap<String, Ability>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SummonerSpell {
    display_name: String,
    raw_description: String,
    raw_display_name: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SummonerSpells {
    summoner_spell_one: SummonerSpell,
    summoner_spell_two: SummonerSpell,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Rune {
    display_name: String,
    id: i32,
    raw_description: String,
    raw_display_name: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StatRune {
    id: i32,
    raw_description: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MainRunes {
    keystone: Rune,
    primary_rune_tree: Rune,
    secondary_rune_tree: Rune,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActivePlayerRunes {
    #[serde(flatten)]
    main_runes: MainRunes,
    general_runes: Vec<Rune>,
    stat_runes: Vec<StatRune>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GameStats {
    game_mode: String,
    game_time: f64,
    map_name: String,
    map_number: i32,
    map_terrain: String,
}

//TODO: Try and rework to use PascalCase?
#[derive(Debug, Serialize, Deserialize)]
pub struct GameEvent {
    #[serde(rename = "EventID")]
    event_id: i32,
    #[serde(rename = "EventTime")]
    event_time: f64,
    #[serde(flatten)]
    event_type: GameEventType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GameEventAll {
    #[serde(rename = "Events")]
    events: Vec<GameEvent>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "EventName")]
pub enum GameEventType {
    GameStart,
    MinionsSpawning,
    ChampionKill {
        #[serde(rename = "KillerName")]
        killer_name: String,
        #[serde(rename = "VictimName")]
        victim_name: String,
        #[serde(rename = "Assisters")]
        assisters: Vec<String>,
    },
    FirstBrick {
        #[serde(rename = "KillerName")]
        killer_name: String,
    },
    TurretKilled {
        #[serde(rename = "KillerName")]
        killer_name: String,
        #[serde(rename = "TurretKilled")]
        turret_killed: String,
        #[serde(rename = "Assisters")]
        assisters: Vec<String>,
    },
    InhibKilled {
        #[serde(rename = "KillerName")]
        killer_name: String,
        #[serde(rename = "InhibKilled")]
        inhib_killed: String,
        #[serde(rename = "Assisters")]
        assisters: Vec<String>,
    },
    DragonKill {
        #[serde(rename = "KillerName")]
        killer_name: String,
        #[serde(rename = "DragonType")]
        dragon_type: String,
        #[serde(deserialize_with = "bool_from_string")]
        #[serde(rename = "Stolen")]
        stolen: bool,
        #[serde(rename = "Assisters")]
        assisters: Vec<String>,
    },
    HeraldKill {
        #[serde(rename = "KillerName")]
        killer_name: String,
        #[serde(deserialize_with = "bool_from_string")]
        #[serde(rename = "Stolen")]
        stolen: bool,
        #[serde(rename = "Assisters")]
        assisters: Vec<String>,
    },
    BaronKill {
        #[serde(rename = "KillerName")]
        killer_name: String,
        #[serde(deserialize_with = "bool_from_string")]
        #[serde(rename = "Stolen")]
        stolen: bool,
        #[serde(rename = "Assisters")]
        assisters: Vec<String>,
    },
    Multikill {
        #[serde(rename = "KillerName")]
        killer_name: String,
        #[serde(rename = "KillStreak")]
        killstreak: i32,
    },
    Ace {
        #[serde(rename = "Acer")]
        acer: String,
        #[serde(rename = "AcingTeam")]
        acing_team: String,
    },
}

fn bool_from_string<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    match String::deserialize(deserializer)?.as_ref() {
        "True" => Ok(true),
        "False" => Ok(false),
        other => Err(de::Error::invalid_value(
            de::Unexpected::Str(other),
            &"OK or nOK",
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn item() {
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
    fn item_list() {
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
    fn score() {
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
    fn gamestats() {
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

    #[test]
    fn event_kill() {
        let event_str = r#"{
            "Assisters": [
                "Player1",
                "Player2"
            ],
            "EventID": 2,
            "EventName": "ChampionKill",
            "EventTime": 154.3977508544922,
            "KillerName": "BRS",
            "VictimName": "Lux Bot"
        }"#;
        let e: GameEvent = serde_json::from_str(event_str).unwrap();
        match e.event_type {
            GameEventType::ChampionKill {
                killer_name,
                assisters,
                ..
            } => {
                assert_eq!(killer_name, "BRS");
                assert_eq!(assisters[0], "Player1");
            }
            _ => (),
        }
    }

    #[test]
    fn event_minions() {
        let event_str = r#"{
            "EventID": 0,
            "EventName": "MinionsSpawning",
            "EventTime": 0
        }"#;
        let e: GameEvent = serde_json::from_str(event_str).unwrap();
        match e.event_type {
            GameEventType::MinionsSpawning => (),
            _ => panic!("Did not match MinionsSpawning"),
        }
    }

    #[test]
    fn event_dragon() {
        let event_str = r#"{
            "EventID": 0,
            "EventName": "DragonKill",
            "EventTime": 0,
            "DragonType": "Elder",
            "Stolen": "False",
            "KillerName": "Riot Tuxedo",
            "Assisters": [
                "Player 1"
            ]
        }"#;
        let e: GameEvent = serde_json::from_str(event_str).unwrap();
        match e.event_type {
            GameEventType::DragonKill {
                killer_name,
                stolen,
                ..
            } => {
                assert_eq!(stolen, false);
                assert_eq!(killer_name, "Riot Tuxedo");
            }
            _ => panic!("Did not match DragonKill"),
        }
    }

    #[test]
    fn all_events() {
        let event_list_str = r#"{
            "Events": [
                {
                    "EventID": 0,
                    "EventName": "GameStart",
                    "EventTime": 0
                },
                {
                    "EventID": 0,
                    "EventName": "MinionsSpawning",
                    "EventTime": 0
                },
                {
                    "EventID": 0,
                    "EventName": "FirstBrick",
                    "EventTime": 0,
                    "KillerName": "Riot Tuxedo"
                },
                {
                    "EventID": 0,
                    "EventName": "TurretKilled",
                    "EventTime": 0,
                    "TurretKilled": "Turret_T2_L_03_A",
                    "KillerName": "Riot Tuxedo",
                    "Assisters": [
                        "Player 1"
                    ]
                },
                {
                    "EventID": 0,
                    "EventName": "InhibKilled",
                    "EventTime": 0,
                    "InhibKilled": "Barracks_T2_R1",
                    "KillerName": "Riot Tuxedo",
                    "Assisters": [
                        "Player 1"
                    ]
                },
                {
                    "EventID": 0,
                    "EventName": "DragonKill",
                    "EventTime": 0,
                    "DragonType": "Earth",
                    "Stolen": "False",
                    "KillerName": "Riot Tuxedo",
                    "Assisters": [
                        "Player 1"
                    ]
                },
                {
                    "EventID": 0,
                    "EventName": "DragonKill",
                    "EventTime": 0,
                    "DragonType": "Elder",
                    "Stolen": "False",
                    "KillerName": "Riot Tuxedo",
                    "Assisters": [
                        "Player 1"
                    ]
                },
                {
                    "EventID": 0,
                    "EventName": "HeraldKill",
                    "EventTime": 0,
                    "Stolen": "False",
                    "KillerName": "Riot Tuxedo",
                    "Assisters": [
                        "Player 1"
                    ]
                },
                {
                    "EventID": 0,
                    "EventName": "BaronKill",
                    "EventTime": 0,
                    "Stolen": "False",
                    "KillerName": "Riot Tuxedo",
                    "Assisters": [
                        "Player 1"
                    ]
                },
                {
                    "EventID": 0,
                    "EventName": "ChampionKill",
                    "EventTime": 0,
                    "VictimName": "Riot Gene",
                    "KillerName": "Riot Tuxedo",
                    "Assisters": [
                        "Player 1"
                    ]
                },
                {
                    "EventID": 0,
                    "EventName": "Multikill",
                    "EventTime": 0,
                    "KillerName": "Riot Tuxedo",
                    "KillStreak": 2
                },
                {
                    "EventID": 0,
                    "EventName": "Multikill",
                    "EventTime": 0,
                    "KillerName": "Riot Tuxedo",
                    "KillStreak": 3
                },
                {
                    "EventID": 0,
                    "EventName": "Multikill",
                    "EventTime": 0,
                    "KillerName": "Riot Tuxedo",
                    "KillStreak": 4
                },
                {
                    "EventID": 0,
                    "EventName": "Multikill",
                    "EventTime": 0,
                    "KillerName": "Riot Tuxedo",
                    "KillStreak": 5
                },
                {
                    "EventID": 0,
                    "EventName": "Ace",
                    "EventTime": 0,
                    "Acer": "Riot Tuxedo",
                    "AcingTeam": "ORDER"
                }
            ]
        }"#;
        let allevents: GameEventAll = serde_json::from_str(event_list_str).unwrap();
        assert_eq!(allevents.events.len(), 15);
    }

    #[test]
    fn main_runes() {
        let mainrunes_str = r#"{
            "keystone": {
                "displayName": "Electrocute",
                "id": 8112,
                "rawDescription": "perk_tooltip_Electrocute",
                "rawDisplayName": "perk_displayname_Electrocute"
            },
            "primaryRuneTree": {
                "displayName": "Domination",
                "id": 8100,
                "rawDescription": "perkstyle_tooltip_7200",
                "rawDisplayName": "perkstyle_displayname_7200"
            },
            "secondaryRuneTree": {
                "displayName": "Sorcery",
                "id": 8200,
                "rawDescription": "perkstyle_tooltip_7202",
                "rawDisplayName": "perkstyle_displayname_7202"
            }
        }"#;
        let mr: MainRunes = serde_json::from_str(mainrunes_str).unwrap();
        assert_eq!(mr.keystone.display_name, "Electrocute");
    }

    #[test]
    fn active_runes() {
        let activerunes_str = r#"{
            "keystone": {
                "displayName": "Electrocute",
                "id": 8112,
                "rawDescription": "perk_tooltip_Electrocute",
                "rawDisplayName": "perk_displayname_Electrocute"
            },
            "primaryRuneTree": {
                "displayName": "Domination",
                "id": 8100,
                "rawDescription": "perkstyle_tooltip_7200",
                "rawDisplayName": "perkstyle_displayname_7200"
            },
            "secondaryRuneTree": {
                "displayName": "Sorcery",
                "id": 8200,
                "rawDescription": "perkstyle_tooltip_7202",
                "rawDisplayName": "perkstyle_displayname_7202"
            },
            "generalRunes": [
                {
                    "displayName": "Electrocute",
                    "id": 8112,
                    "rawDescription": "perk_tooltip_Electrocute",
                    "rawDisplayName": "perk_displayname_Electrocute"
                },
                {
                    "displayName": "Cheap Shot",
                    "id": 8126,
                    "rawDescription": "perk_tooltip_CheapShot",
                    "rawDisplayName": "perk_displayname_CheapShot"
                },
                {
                    "displayName": "Eyeball Collection",
                    "id": 8138,
                    "rawDescription": "perk_tooltip_EyeballCollection",
                    "rawDisplayName": "perk_displayname_EyeballCollection"
                },
                {
                    "displayName": "Relentless Hunter",
                    "id": 8105,
                    "rawDescription": "perk_tooltip_8105",
                    "rawDisplayName": "perk_displayname_8105"
                },
                {
                    "displayName": "Celerity",
                    "id": 8234,
                    "rawDescription": "perk_tooltip_Celerity",
                    "rawDisplayName": "perk_displayname_Celerity"
                },
                {
                    "displayName": "Gathering Storm",
                    "id": 8236,
                    "rawDescription": "perk_tooltip_GatheringStorm",
                    "rawDisplayName": "perk_displayname_GatheringStorm"
                }
            ],
            "statRunes": [
                {
                    "id": 5007,
                    "rawDescription": "perk_tooltip_StatModCooldownReductionScaling"
                },
                {
                    "id": 5008,
                    "rawDescription": "perk_tooltip_StatModAdaptive"
                },
                {
                    "id": 5003,
                    "rawDescription": "perk_tooltip_StatModMagicResist"
                }
            ]
        }"#;
        let ar: ActivePlayerRunes = serde_json::from_str(activerunes_str).unwrap();
        assert_eq!(ar.main_runes.keystone.display_name, "Electrocute");
        assert_eq!(ar.general_runes.len(), 6);
        assert_eq!(ar.stat_runes.len(), 3);
    }

    #[test]
    fn active_abilities() {
        let abilities_str = r#"{
            "E": {
                "abilityLevel": 0,
                "displayName": "Molten Shield",
                "id": "AnnieE",
                "rawDescription": "GeneratedTip_Spell_AnnieE_Description",
                "rawDisplayName": "GeneratedTip_Spell_AnnieE_DisplayName"
            },
            "Passive": {
                "displayName": "Pyromania",
                "id": "AnniePassive",
                "rawDescription": "GeneratedTip_Passive_AnniePassive_Description",
                "rawDisplayName": "GeneratedTip_Passive_AnniePassive_DisplayName"
            },
            "Q": {
                "abilityLevel": 0,
                "displayName": "Disintegrate",
                "id": "AnnieQ",
                "rawDescription": "GeneratedTip_Spell_AnnieQ_Description",
                "rawDisplayName": "GeneratedTip_Spell_AnnieQ_DisplayName"
            },
            "R": {
                "abilityLevel": 0,
                "displayName": "Summon: Tibbers",
                "id": "AnnieR",
                "rawDescription": "GeneratedTip_Spell_AnnieR_Description",
                "rawDisplayName": "GeneratedTip_Spell_AnnieR_DisplayName"
            },
            "W": {
                "abilityLevel": 0,
                "displayName": "Incinerate",
                "id": "AnnieW",
                "rawDescription": "GeneratedTip_Spell_AnnieW_Description",
                "rawDisplayName": "GeneratedTip_Spell_AnnieW_DisplayName"
            }
        }"#;
        let apa: ActivePlayerAbilities = serde_json::from_str(abilities_str).unwrap();

        assert_eq!(apa.abilities.len(), 5);
    }

    #[test]
    fn active_player() {
        let active_player_str = r#"{
            "abilities": {
                "E": {
                    "abilityLevel": 0,
                    "displayName": "Molten Shield",
                    "id": "AnnieE",
                    "rawDescription": "GeneratedTip_Spell_AnnieE_Description",
                    "rawDisplayName": "GeneratedTip_Spell_AnnieE_DisplayName"
                },
                "Passive": {
                    "displayName": "Pyromania",
                    "id": "AnniePassive",
                    "rawDescription": "GeneratedTip_Passive_AnniePassive_Description",
                    "rawDisplayName": "GeneratedTip_Passive_AnniePassive_DisplayName"
                },
                "Q": {
                    "abilityLevel": 0,
                    "displayName": "Disintegrate",
                    "id": "AnnieQ",
                    "rawDescription": "GeneratedTip_Spell_AnnieQ_Description",
                    "rawDisplayName": "GeneratedTip_Spell_AnnieQ_DisplayName"
                },
                "R": {
                    "abilityLevel": 0,
                    "displayName": "Summon: Tibbers",
                    "id": "AnnieR",
                    "rawDescription": "GeneratedTip_Spell_AnnieR_Description",
                    "rawDisplayName": "GeneratedTip_Spell_AnnieR_DisplayName"
                },
                "W": {
                    "abilityLevel": 0,
                    "displayName": "Incinerate",
                    "id": "AnnieW",
                    "rawDescription": "GeneratedTip_Spell_AnnieW_Description",
                    "rawDisplayName": "GeneratedTip_Spell_AnnieW_DisplayName"
                }
            },
            "championStats": {
                "abilityPower": 0,
                "armor": 0,
                "armorPenetrationFlat": 0,
                "armorPenetrationPercent": 0,
                "attackDamage": 0,
                "attackRange": 0,
                "attackSpeed": 0,
                "bonusArmorPenetrationPercent": 0,
                "bonusMagicPenetrationPercent": 0,
                "cooldownReduction": 0,
                "critChance": 0,
                "critDamage": 0,
                "currentHealth": 0,
                "healthRegenRate": 0,
                "lifeSteal": 0,
                "magicLethality": 0,
                "magicPenetrationFlat": 0,
                "magicPenetrationPercent": 0,
                "magicResist": 0,
                "maxHealth": 0,
                "moveSpeed": 0,
                "physicalLethality": 0,
                "resourceMax": 0,
                "resourceRegenRate": 0,
                "resourceType": "MANA",
                "resourceValue": 0,
                "spellVamp": 0,
                "tenacity": 0
            },
            "currentGold": 0,
            "fullRunes": {
                "generalRunes": [
                    {
                        "displayName": "Electrocute",
                        "id": 8112,
                        "rawDescription": "perk_tooltip_Electrocute",
                        "rawDisplayName": "perk_displayname_Electrocute"
                    },
                    {
                        "displayName": "Cheap Shot",
                        "id": 8126,
                        "rawDescription": "perk_tooltip_CheapShot",
                        "rawDisplayName": "perk_displayname_CheapShot"
                    },
                    {
                        "displayName": "Eyeball Collection",
                        "id": 8138,
                        "rawDescription": "perk_tooltip_EyeballCollection",
                        "rawDisplayName": "perk_displayname_EyeballCollection"
                    },
                    {
                        "displayName": "Relentless Hunter",
                        "id": 8105,
                        "rawDescription": "perk_tooltip_8105",
                        "rawDisplayName": "perk_displayname_8105"
                    },
                    {
                        "displayName": "Celerity",
                        "id": 8234,
                        "rawDescription": "perk_tooltip_Celerity",
                        "rawDisplayName": "perk_displayname_Celerity"
                    },
                    {
                        "displayName": "Gathering Storm",
                        "id": 8236,
                        "rawDescription": "perk_tooltip_GatheringStorm",
                        "rawDisplayName": "perk_displayname_GatheringStorm"
                    }
                ],
                "keystone": {
                    "displayName": "Electrocute",
                    "id": 8112,
                    "rawDescription": "perk_tooltip_Electrocute",
                    "rawDisplayName": "perk_displayname_Electrocute"
                },
                "primaryRuneTree": {
                    "displayName": "Domination",
                    "id": 8100,
                    "rawDescription": "perkstyle_tooltip_7200",
                    "rawDisplayName": "perkstyle_displayname_7200"
                },
                "secondaryRuneTree": {
                    "displayName": "Sorcery",
                    "id": 8200,
                    "rawDescription": "perkstyle_tooltip_7202",
                    "rawDisplayName": "perkstyle_displayname_7202"
                },
                "statRunes": [
                    {
                        "id": 5008,
                        "rawDescription": "perk_tooltip_StatModAdaptive"
                    },
                    {
                        "id": 5003,
                        "rawDescription": "perk_tooltip_StatModMagicResist"
                    },
                    {
                        "id": 5003,
                        "rawDescription": "perk_tooltip_StatModMagicResist"
                    }
                ]
            },
            "level": 1,
            "summonerName": "Riot Tuxedo"
        }"#;

        let ap: ActivePlayer = serde_json::from_str(active_player_str).unwrap();

        assert_eq!(ap.summoner_name, "Riot Tuxedo");
        assert_eq!(ap.full_runes.general_runes.len(), 6);
    }

    #[test]
    fn player() {
        let player_str = r#"{
            "championName": "Annie",
            "isBot": false,
            "isDead": false,
            "items": [],
            "level": 1,
            "position": "",
            "rawChampionName": "game_character_displayname_Annie",
            "respawnTimer": 0,
            "runes": {
                "keystone": {
                    "displayName": "Electrocute",
                    "id": 8112,
                    "rawDescription": "perk_tooltip_Electrocute",
                    "rawDisplayName": "perk_displayname_Electrocute"
                },
                "primaryRuneTree": {
                    "displayName": "Domination",
                    "id": 8100,
                    "rawDescription": "perkstyle_tooltip_7200",
                    "rawDisplayName": "perkstyle_displayname_7200"
                },
                "secondaryRuneTree": {
                    "displayName": "Sorcery",
                    "id": 8200,
                    "rawDescription": "perkstyle_tooltip_7202",
                    "rawDisplayName": "perkstyle_displayname_7202"
                }
            },
            "scores": {
                "assists": 0,
                "creepScore": 0,
                "deaths": 0,
                "kills": 0,
                "wardScore": 0
            },
            "skinID": 0,
            "summonerName": "Riot Tuxedo",
            "summonerSpells": {
                "summonerSpellOne": {
                    "displayName": "Flash",
                    "rawDescription": "GeneratedTip_SummonerSpell_SummonerFlash_Description",
                    "rawDisplayName": "GeneratedTip_SummonerSpell_SummonerFlash_DisplayName"
                },
                "summonerSpellTwo": {
                    "displayName": "Ignite",
                    "rawDescription": "GeneratedTip_SummonerSpell_SummonerDot_Description",
                    "rawDisplayName": "GeneratedTip_SummonerSpell_SummonerDot_DisplayName"
                }
            },
            "team": "ORDER"
        }"#;

        let p: Player = serde_json::from_str(player_str).unwrap();

        assert_eq!(p.summoner_name, "Riot Tuxedo");
    }
}
