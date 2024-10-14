pub mod fpl_data {
    use reqwest;
    use serde::{Deserialize, Serialize};


    pub trait FplApiData {

        fn api_endpoint() -> String;
        fn json_field() -> String;
        type JsonExtractType;
        fn from_json(json: serde_json::Value) -> Result<Self::JsonExtractType, String>;
    }

    fn convert_vec_to_generic<F, T>(values: Vec<serde_json::Value>, conversion_fn: F) -> T::JsonExtractType
    where
        T: FplApiData,
        T::JsonExtractType: FromIterator<T>,
        F: Fn(serde_json::Value) -> T,
    {
        values.into_iter().map(conversion_fn).collect()
    }

    async fn api_call(url: &str) -> Result<serde_json::Value, reqwest::Error> {
        let response = reqwest::get(url).await?;

        // Return the reponse directly
        response.json().await
    }

    pub async fn get_component<T>() -> Result<T::JsonExtractType, String>
    where
        T: FplApiData
    {
        let endpoint = T::api_endpoint();
        let data = api_call(&endpoint).await.expect("Failed to get data from API call");

        match data {
            serde_json::Value::Array(_) => T::from_json(data),
            serde_json::Value::Object(mut object_data) => {
                if let Some(field_value) = object_data.remove(&T::json_field()) {
                    T::from_json(field_value)
                }
                else {
                    Err(format!("Field {} not found in JSON object", T::json_field()))
                }
            },
            _ => Err(format!("Expected data to be a json object"))
        }
    }

    pub async fn get_all_data() -> Result<serde_json::Value, reqwest::Error> {
        let request_url = "https://fantasy.premierleague.com/api/bootstrap-static/";
        api_call(&request_url).await
    }

    pub async fn get_events() -> Result<Vec<FplApiGameweek>, String> {
        get_component::<FplApiGameweek>().await
    }

    pub async fn get_players() -> Result<Vec<FplApiPlayer>, String> {
        get_component::<FplApiPlayer>().await
    }

    pub async fn get_positions() -> Result<Vec<FplApiPosition>, String> {
        get_component::<FplApiPosition>().await
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

    pub async fn get_teams() -> Result<Vec<FplApiTeam>, String> {
        get_component::<FplApiTeam>().await
    }

    pub async fn get_fixtures() -> Result<Vec<FplApiFixture>, String> {
        get_component::<FplApiFixture>().await
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

    impl FplApiData for FplApiTeam {
        fn api_endpoint() -> String {
            format!("https://fantasy.premierleague.com/api/bootstrap-static/")
        }

        fn json_field() -> String {
            format!("teams")
        }

        fn from_json(json: serde_json::Value) -> Result<Self::JsonExtractType, String> {

            // The team list is in "teams"
            if let serde_json::Value::Array(teams_list) = json {
                let team_conversion = |json_value: serde_json::Value| -> FplApiTeam {
                    serde_json::from_value(json_value).expect("Failed to convert team")
                };

                Ok(teams_list.into_iter().map(team_conversion).collect())

            } else {
                Err(format!("Expected teams to be a json array"))
            }

        }

        type JsonExtractType = Vec<FplApiTeam>;
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

    impl FplApiData for FplApiPlayer {
        fn api_endpoint() -> String {
            format!("https://fantasy.premierleague.com/api/bootstrap-static/")
        }

        fn json_field() -> String {
            format!("elements")
        }

        fn from_json(json: serde_json::Value) -> Result<Vec<FplApiPlayer>, String> {
            
            if let serde_json::Value::Array(players_list) = json {
                let player_conversion = |json_value: serde_json::Value| -> FplApiPlayer {
                    serde_json::from_value(json_value).expect("Failed to convert player")
                };
                Ok(convert_vec_to_generic(players_list, player_conversion))
            } else {
                Err(format!("Expected players to be a json array"))
            }

        }

        type JsonExtractType = Vec<FplApiPlayer>;
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

    impl FplApiData for FplApiPosition {
        fn api_endpoint() -> String {
            format!("https://fantasy.premierleague.com/api/bootstrap-static/")
        }

        fn json_field() -> String {
            format!("element_types")
        }

        fn from_json(json: serde_json::Value) -> Result<Vec<FplApiPosition>, String> {
        
            // The team list is in "teams"
            if let serde_json::Value::Array(position_list) = json {
                let position_conversion = |json_value: serde_json::Value| -> FplApiPosition {
                    serde_json::from_value(json_value).expect("Failed to convert team")
                };

                Ok(position_list.into_iter().map(position_conversion).collect())

            } else {
                Err(format!("Expected positions to be a json array"))
            }
        }

        type JsonExtractType = Vec<FplApiPosition>;
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

    impl FplApiData for FplApiFixture {
        fn api_endpoint() -> String {
            format!("https://fantasy.premierleague.com/api/fixtures/")
        }

        fn from_json(json: serde_json::Value) -> Result<Self::JsonExtractType, String> {
            if let serde_json::Value::Array(fixtures) = json {

                let fixture_conversion = |json_value: serde_json::Value| -> FplApiFixture {
                    serde_json::from_value(json_value).expect("Failed to convert fixtures")
                };

                Ok(fixtures.into_iter().map(fixture_conversion).collect())

            } else {
                Err(format!("Expected fixtures to be a json array"))
            }
        }

        fn json_field() -> String {
            format!("")
        }

        type JsonExtractType = Vec<Self>;
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct FplApiChipPlay {
        chip_name: String,
        num_played: u32,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct FplApiTopElementInfo {
        id: Option<u32>,
        points: Option<u32>,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct FplApiGameweek {
        id: u32,
        name: String,
        deadline_time: String,
        release_time: Option<String>,
        average_entry_score: u32,
        finished: bool,
        data_checked: bool,
        highest_scoring_entry: Option<u32>,
        deadline_time_epoch: u64,
        deadline_time_game_offset: u32,
        highest_score: Option<u32>,
        is_previous: bool,
        is_current: bool,
        is_next: bool,
        cup_leagues_created: bool,
        h2h_ko_matches_created: bool,
        ranked_count: u64,
        chip_plays: Vec<FplApiChipPlay>,
        most_selected: Option<u32>,
        most_transferred_in: Option<u32>,
        top_element: Option<u32>,
        top_element_info: Option<FplApiTopElementInfo>,
        transfers_made: u64,
        most_captained: Option<u32>,
        most_vice_captained: Option<u32>,
    }

    impl FplApiData for FplApiGameweek {

        fn api_endpoint() -> String {
            format!("https://fantasy.premierleague.com/api/bootstrap-static/")
        }

        fn json_field() -> String {
            format!("events")
        }

        fn from_json(json: serde_json::Value) -> Result<Self::JsonExtractType, String> {

            // The team list is in "events"
            if let serde_json::Value::Array(gameweek_list) = json {
                let event_conversion = |json_value: serde_json::Value| -> FplApiGameweek {
                    serde_json::from_value(json_value).expect("Failed to convert gameweek")
                };

                Ok(gameweek_list.into_iter().map(event_conversion).collect())

            } else {
                Err(format!("Expected events to be a json array"))
            }

        }

        type JsonExtractType = Vec<FplApiGameweek>;
    }

}

#[cfg(test)]
mod tests {
    use super::fpl_data;

    #[tokio::test]
    async fn test_get_data() {
        let _ = fpl_data::get_all_data().await;
    }
}
