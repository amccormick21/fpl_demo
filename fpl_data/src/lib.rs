pub mod fpl_data {
    use reqwest;
    use serde::{Deserialize, Serialize};

    fn convert_vec_to_generic<F, T>(values: Vec<serde_json::Value>, conversion_fn: F) -> Vec<T>
    where
        T: Serialize + Deserialize<'static>, // 'static lifetime for simplicity
        F: Fn(serde_json::Value) -> T,
    {
        values.into_iter().map(conversion_fn).collect()
    }

    async fn api_call(url: &str) -> Result<serde_json::Value, reqwest::Error> {
        let response = reqwest::get(url).await?;

        // Return the reponse directly
        response.json().await
    }

    pub async fn get_all_data() -> Result<serde_json::Value, reqwest::Error> {
        let request_url = "https://fantasy.premierleague.com/api/bootstrap-static/".to_string();
        api_call(&request_url).await
    }

    pub async fn get_fixtures_data() -> Result<serde_json::Value, reqwest::Error> {
        let request_url = "https://fantasy.premierleague.com/api/fixtures/".to_string();
        api_call(&request_url).await
    }

    pub async fn get_events() -> Result<Vec<serde_json::Value>, &'static str> {
        let all_data = get_all_data()
            .await
            .expect("Failed to get data from API call");

        if let serde_json::Value::Object(object_data) = all_data {
            if let serde_json::Value::Array(events_list) = &object_data["events"] {
                Ok(events_list.to_vec())
            } else {
                Err("Expected events to be a json array")
            }
        } else {
            Err("Expected all data to be a json object")
        }
    }

    pub async fn get_players() -> Result<Vec<FplApiPlayer>, &'static str> {
        let all_data = get_all_data()
            .await
            .expect("Failed to get all data from API call");

        if let serde_json::Value::Object(mut object_data) = all_data {
            if let Some(serde_json::Value::Array(players_list)) = object_data.remove("elements") {
                let player_conversion = |json_value: serde_json::Value| -> FplApiPlayer {
                    serde_json::from_value(json_value).expect("Failed to convert player")
                };
                Ok(convert_vec_to_generic(players_list, player_conversion))
            } else {
                Err("Expected players to be a json array")
            }
        } else {
            Err("Expected all data to be a json object")
        }
    }

    pub async fn get_positions() -> Result<Vec<FplApiPosition>, &'static str> {
        let all_data = get_all_data()
            .await
            .expect("Failed to get all data from API call");

        if let serde_json::Value::Object(mut object_data) = all_data {
            if let Some(serde_json::Value::Array(positions_list)) =
                object_data.remove("element_types")
            {
                let position_conversion = |json_value: serde_json::Value| -> FplApiPosition {
                    serde_json::from_value(json_value).expect("Failed to convert position")
                };
                Ok(convert_vec_to_generic(positions_list, position_conversion))
            } else {
                Err("Expected positions to be a json array")
            }
        } else {
            Err("Expected all data to be a json object")
        }
    }

    pub async fn get_player_count() -> Result<usize, &'static str> {
        let all_data = get_all_data()
            .await
            .expect("Failed to get all data from API call");

        if let serde_json::Value::Object(object_data) = all_data {
            if let serde_json::Value::Number(player_count) = &object_data["total_players"] {
                Ok(player_count.as_u64().unwrap() as usize)
            } else {
                Err("Expected total_players to be a number")
            }
        } else {
            Err("Expected all data to be a json object")
        }
    }

    pub async fn get_teams() -> Result<Vec<FplApiTeam>, &'static str> {
        let all_data = get_all_data()
            .await
            .expect("Failed to get all data from API call");

        if let serde_json::Value::Object(mut object_data) = all_data {
            if let Some(serde_json::Value::Array(teams_list)) = object_data.remove("teams") {
                let team_conversion = |json_value: serde_json::Value| -> FplApiTeam {
                    serde_json::from_value(json_value).expect("Failed to convert team")
                };
                Ok(convert_vec_to_generic(teams_list, team_conversion))
            } else {
                Err("Expected teams to be a json array")
            }
        } else {
            Err("Expected all data to be a json object")
        }
    }

    pub async fn get_fixtures() -> Result<Vec<FplApiFixture>, &'static str> {
        let fixtures_data = get_fixtures_data()
            .await
            .expect("Failed to get data from API call");

        if let serde_json::Value::Array(fixtures) = fixtures_data {
            let fixture_conversion = |json_value: serde_json::Value| -> FplApiFixture {
                serde_json::from_value(json_value).expect("Failed to convert fixtures")
            };
            Ok(convert_vec_to_generic(fixtures, fixture_conversion))
        } else {
            Err("Expected fixtures to be a json array")
        }
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct FplApiTeam {
        pub id: u64,
        pub code: u64,
        pub name: String,
        pub short_name: String,
        pub pulse_id: u32,
        pub strength: u32,
        pub played: u32,
        pub win: u32,
        pub draw: u32,
        pub loss: u32,
        pub points: u32,
        pub position: u32,
        pub strength_overall_home: u32,
        pub strength_overall_away: u32,
        pub strength_attack_home: u32,
        pub strength_attack_away: u32,
        pub strength_defence_home: u32,
        pub strength_defence_away: u32,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct FplApiPlayer {
        pub chance_of_playing_next_round: Option<u32>,
        pub chance_of_playing_this_round: Option<u32>,
        pub code: u32,
        pub cost_change_event: i32,
        pub cost_change_event_fall: i32,
        pub cost_change_start: i32,
        pub cost_change_start_fall: i32,
        pub dreamteam_count: u32,
        pub element_type: u32,
        pub ep_next: Option<String>,
        pub ep_this: Option<String>,
        pub event_points: i32,
        pub first_name: String,
        pub form: String,
        pub id: u32,
        pub in_dreamteam: bool,
        pub news: String,
        pub news_added: Option<String>,
        pub now_cost: u32,
        pub photo: String,
        pub points_per_game: String,
        pub second_name: String,
        pub selected_by_percent: String,
        pub special: bool,
        pub squad_number: Option<u32>,
        pub status: String,
        pub team: u32,
        pub team_code: u32,
        pub total_points: i32,
        pub transfers_in: u32,
        pub transfers_in_event: u32,
        pub transfers_out: u32,
        pub transfers_out_event: u32,
        pub value_form: String,
        pub value_season: String,
        pub web_name: String,
        pub minutes: u32,
        pub goals_scored: u32,
        pub assists: u32,
        pub clean_sheets: u32,
        pub goals_conceded: u32,
        pub own_goals: u32,
        pub penalties_saved: u32,
        pub penalties_missed: u32,
        pub yellow_cards: u32,
        pub red_cards: u32,
        pub saves: u32,
        pub bonus: u32,
        pub bps: i32,
        pub influence: String,
        pub creativity: String,
        pub threat: String,
        pub ict_index: String,
        pub starts: u32,
        pub expected_goals: String,
        pub expected_assists: String,
        pub expected_goal_involvements: String,
        pub expected_goals_conceded: String,
        pub influence_rank: u32,
        pub influence_rank_type: u32,
        pub creativity_rank: u32,
        pub creativity_rank_type: u32,
        pub threat_rank: u32,
        pub threat_rank_type: u32,
        pub ict_index_rank: u32,
        pub ict_index_rank_type: u32,
        pub corners_and_indirect_freekicks_order: Option<u32>,
        pub corners_and_indirect_freekicks_text: String,
        pub direct_freekicks_order: Option<u32>,
        pub direct_freekicks_text: String,
        pub penalties_order: Option<u32>,
        pub penalties_text: String,
        pub expected_goals_per_90: f64,
        pub saves_per_90: f64,
        pub expected_assists_per_90: f64,
        pub expected_goal_involvements_per_90: f64,
        pub expected_goals_conceded_per_90: f64,
        pub goals_conceded_per_90: f64,
        pub now_cost_rank: u32,
        pub now_cost_rank_type: u32,
        pub form_rank: u32,
        pub form_rank_type: u32,
        pub points_per_game_rank: u32,
        pub points_per_game_rank_type: u32,
        pub selected_rank: u32,
        pub selected_rank_type: u32,
        pub starts_per_90: f64,
        pub clean_sheets_per_90: f64,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct FplApiPosition {
        pub id: u32,
        pub plural_name: String,
        pub plural_name_short: String,
        pub singular_name: String,
        pub singular_name_short: String,
        pub squad_select: u32,
        pub squad_min_play: u32,
        pub squad_max_play: u32,
        pub ui_shirt_specific: bool,
        pub sub_positions_locked: Vec<u32>,
        pub element_count: u32,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct FplApiFixturePlayerStat {
        pub value: i32,
        pub element: u32, // Player (element) id
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct FplApiFixtureStats {
        pub identifier: String,
        pub a: Vec<FplApiFixturePlayerStat>, // Away team
        pub h: Vec<FplApiFixturePlayerStat>, // Home team
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct FplApiFixture {
        pub code: u32,
        pub event: u32,
        pub finished: bool,
        pub finished_provisional: bool,
        pub id: u32,
        pub kickoff_time: String,
        pub minutes: u32,
        pub provisional_start_time: bool,
        pub started: bool,
        pub team_a: u64,
        pub team_a_score: Option<u32>,
        pub team_h: u64,
        pub team_h_score: Option<u32>,
        pub stats: Vec<FplApiFixtureStats>,
    }
}

#[cfg(test)]
mod tests {
    use super::fpl_data;

    #[tokio::test]
    async fn test_get_data() {
        let _ = fpl_data::get_all_data().await;
    }

    #[tokio::test]
    async fn test_get_events() {
        let events = fpl_data::get_events().await.expect("Failed to get events");

        if let Some(serde_json::Value::Object(first_event)) = events.first() {
            if let serde_json::Value::String(event_name) = &first_event["name"] {
                assert_eq!(event_name, "Gameweek 1");
            }
        }
    }
}
