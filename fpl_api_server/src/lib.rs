use reqwest;

pub async fn get_all_data() -> Result<serde_json::Value, reqwest::Error> {
    let request_url = format!("https://fantasy.premierleague.com/api/bootstrap-static/");
    let response = reqwest::get(&request_url).await?;

    // Return the reponse directly
    response.json().await
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

pub async fn get_players() -> Result<Vec<serde_json::Value>, &'static str> {
    let all_data = get_all_data()
        .await
        .expect("Failed to get all data from API call");

    if let serde_json::Value::Object(object_data) = all_data {
        if let serde_json::Value::Array(players_list) = &object_data["elements"] {
            Ok(players_list.to_vec())
        } else {
            Err("Expected players to be a json array")
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

pub async fn get_teams() -> Result<Vec<serde_json::Value>, &'static str> {
    let all_data = get_all_data()
        .await
        .expect("Failed to get all data from API call");

    if let serde_json::Value::Object(object_data) = all_data {
        if let serde_json::Value::Array(events_list) = &object_data["teams"] {
            Ok(events_list.to_vec())
        } else {
            Err("Expected teams to be a json array")
        }
    } else {
        Err("Expected all data to be a json object")
    }
}

mod fpl_players {
    #[derive(Debug, PartialEq)]
    pub struct FplPlayerName {
        pub first_name: String,
        pub second_name: String,
        pub display_name: String,
    }

    #[derive(Debug, PartialEq)]
    pub struct FplPlayerStats {
        pub minutes: u32,
        pub goals_scored: u32,
        pub assists: u32,
        pub clean_sheets: u32,
        pub goals_conceded: u32,
        pub own_goals: u32,
        pub yellow_cards: u32,
        pub red_cards: u32,
        pub saves: u32,
        pub starts: u32,
    }

    impl FplPlayerStats {
        fn nineties(&self) -> f64 {
            self.minutes as f64 / 90.0
        }
        
        pub fn get_stats_per_90(&self) -> FplPlayerStatsPer90 {
            let nineties = self.nineties();
            if nineties == 0.0
            {
                FplPlayerStatsPer90 {
                    starts: 0.0,
                    goals: 0.0,
                    goals_conceded: 0.0,
                    clean_sheets: 0.0
                }
            }
            else
            {
                FplPlayerStatsPer90 {
                    starts: self.starts as f64 / nineties,
                    goals: self.goals_scored as f64 / nineties,
                    goals_conceded: self.goals_conceded as f64 / nineties,
                    clean_sheets: self.clean_sheets as f64 / nineties,
                }
            }
        }
    }

    #[derive(Debug, PartialEq)]
    pub struct FplPlayerStatsPer90 {
        pub starts: f64,
        pub goals: f64,
        pub goals_conceded: f64,
        pub clean_sheets: f64,
    }
    
    impl approx::AbsDiffEq for FplPlayerStatsPer90 {
        type Epsilon = f64;
    
        fn default_epsilon() -> Self::Epsilon {
            f64::default_epsilon()
        }
    
        fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
            f64::abs_diff_eq(&self.starts, &other.starts, epsilon) &&
            f64::abs_diff_eq(&self.goals, &other.goals, epsilon) &&
            f64::abs_diff_eq(&self.goals_conceded, &other.goals_conceded, epsilon) &&
            f64::abs_diff_eq(&self.clean_sheets, &other.clean_sheets, epsilon)
        }
    }
    
    impl approx::RelativeEq for FplPlayerStatsPer90 {
        fn default_max_relative() -> Self::Epsilon {
            f64::default_max_relative()
        }
    
        fn relative_eq(&self, other: &Self, epsilon: Self::Epsilon, max_relative: Self::Epsilon) -> bool {
            f64::relative_eq(&self.starts, &other.starts, epsilon, max_relative) &&
            f64::relative_eq(&self.goals, &other.goals, epsilon, max_relative) &&
            f64::relative_eq(&self.goals_conceded, &other.goals_conceded, epsilon, max_relative) &&
            f64::relative_eq(&self.clean_sheets, &other.clean_sheets, epsilon, max_relative)
        }
    }
    
    impl approx::UlpsEq for FplPlayerStatsPer90 {
        fn default_max_ulps() -> u32 {
            f64::default_max_ulps()
        }
    
        fn ulps_eq(&self, other: &Self, epsilon: Self::Epsilon, max_ulps: u32) -> bool {
            f64::ulps_eq(&self.starts, &other.starts, epsilon, max_ulps) &&
            f64::ulps_eq(&self.goals, &other.goals, epsilon, max_ulps) &&
            f64::ulps_eq(&self.goals_conceded, &other.goals_conceded, epsilon, max_ulps) &&
            f64::ulps_eq(&self.clean_sheets, &other.clean_sheets, epsilon, max_ulps)
        }
    }
    

    #[derive(Debug, PartialEq)]
    pub struct FplPlayerExpectations {
        pub expected_goals: f64,
        pub expected_assists: f64,
        pub expected_goal_involvements: f64,
        pub expected_goals_conceded: f64,
        pub expected_clean_sheets: f64,
    }

    #[derive(Debug, PartialEq)]
    pub struct FplPlayerPointsRecord {
        pub total_points: i32,
        pub bps: i32,
        pub event_points: i32,
    }

    #[derive(Debug, PartialEq)]
    pub struct FplPlayer {
        pub id: u32,
        pub name: FplPlayerName,
        pub stats: FplPlayerStats,
        pub stats_per_90: FplPlayerStatsPer90,
        pub expected_stats: FplPlayerExpectations,
        pub points_record: FplPlayerPointsRecord,
    }

    impl FplPlayer {
        pub fn get_stats_per_90(&self) -> FplPlayerStatsPer90 {
            self.stats.get_stats_per_90()
        }
    }
}

mod fpl_teams {
    #[derive(Debug)]
    pub struct FplTeamTableData {
        pub played: u32,
        pub win: u32,
        pub draw: u32,
        pub loss: u32,
        pub points: u32,
        pub position: u32,
    }

    #[derive(Debug)]
    pub struct FplTeamStrength {
        pub overall_home: u32,
        pub overall_away: u32,
        pub attack_home: u32,
        pub attack_away: u32,
        pub defence_home: u32,
        pub defence_away: u32,
    }

    #[derive(Debug)]
    pub struct FplTeam {
        pub id: u64,
        pub name: String,
        pub short_name: String,
        pub table_data: FplTeamTableData,
        pub strength: FplTeamStrength,
    }
}

mod fpl_conversions {
    use crate::fpl_players;
    use crate::fpl_teams;

    pub fn convert_player(json_player: &serde_json::Value) -> Result<fpl_players::FplPlayer, &str> {
        let api_player: fpl_data::fpl_data::FplApiPlayer =
            serde_json::from_value(json_player.clone()).expect("Failed to convert player");

        Ok(fpl_players::FplPlayer {
            id: api_player.id,
            name: fpl_players::FplPlayerName {
                first_name: api_player.first_name,
                second_name: api_player.second_name,
                display_name: api_player.web_name,
            },
            stats: fpl_players::FplPlayerStats {
                minutes: api_player.minutes,
                goals_scored: api_player.goals_scored,
                assists: api_player.assists,
                clean_sheets: api_player.clean_sheets,
                goals_conceded: api_player.goals_conceded,
                own_goals: api_player.own_goals,
                yellow_cards: api_player.yellow_cards,
                red_cards: api_player.red_cards,
                saves: api_player.saves,
                starts: api_player.starts,
            },
            stats_per_90: fpl_players::FplPlayerStatsPer90 {
                starts: api_player.starts_per_90,
                goals:
                    // We have to manually generate this because for some reason the API won't provide it
                    // And the xG is a different value
                    if api_player.minutes == 0 { 0.0 }
                    else { (api_player.goals_scored as f64) / ((api_player.minutes as f64) / 90.0)},
                goals_conceded: api_player.goals_conceded_per_90,
                clean_sheets: api_player.clean_sheets_per_90,
            },
            expected_stats: fpl_players::FplPlayerExpectations {
                expected_goals: api_player
                    .expected_goals
                    .parse()
                    .expect("Expected goals was not a number"),
                expected_assists: api_player
                    .expected_assists
                    .parse()
                    .expect("Expected assists was not a number"),
                expected_goal_involvements: api_player
                    .expected_goal_involvements
                    .parse()
                    .expect("Expected goal involvements was not a number"),
                expected_goals_conceded: api_player
                    .expected_goals_conceded
                    .parse()
                    .expect("Expected goals conceded was not a number"),
                expected_clean_sheets: api_player
                    .expected_goals_conceded
                    .parse()
                    .expect("Expected goals conceded was not a number"),
            },
            points_record: fpl_players::FplPlayerPointsRecord {
                total_points: api_player.total_points,
                bps: api_player.bps,
                event_points: api_player.event_points,
            },
        })
    }

    pub fn convert_team(json_team: &serde_json::Value) -> Result<fpl_teams::FplTeam, &str> {
        let api_team: fpl_data::fpl_data::FplApiTeam =
            serde_json::from_value(json_team.clone()).expect("Failed to convert team");

        Ok(fpl_teams::FplTeam {
            id: api_team.id,
            name: api_team.name,
            short_name: api_team.short_name,
            table_data: fpl_teams::FplTeamTableData {
                played: api_team.played,
                win: api_team.win,
                draw: api_team.draw,
                loss: api_team.loss,
                points: api_team.points,
                position: api_team.position,
            },
            strength: fpl_teams::FplTeamStrength {
                overall_home: api_team.strength_overall_home,
                overall_away: api_team.strength_overall_away,
                attack_home: api_team.strength_attack_home,
                attack_away: api_team.strength_attack_away,
                defence_home: api_team.strength_defence_home,
                defence_away: api_team.strength_defence_away,
            },
        })
    }
}

#[cfg(test)]
mod tests {

    use approx::assert_relative_eq;

    use super::*;

    #[tokio::test]
    async fn test_get_data() {
        let _ = get_all_data().await;
    }

    #[tokio::test]
    async fn test_get_events() {
        let events = get_events().await.expect("Failed to get events");

        if let Some(serde_json::Value::Object(first_event)) = events.first() {
            if let serde_json::Value::String(event_name) = &first_event["name"] {
                assert_eq!(event_name, "Gameweek 1");
            }
        }
    }

    #[tokio::test]
    async fn test_get_players() {
        let json_players = get_players().await.expect("Failed to get players");

        let players: Vec<fpl_players::FplPlayer> = json_players
            .iter()
            .map(|player| {
                fpl_conversions::convert_player(player).expect("Failed to convert player")
            })
            .collect();

        assert_eq!(players[3].name.display_name, "Fábio Vieira");
    }

    #[tokio::test]
    async fn test_stats_per_90() {
        let json_players = get_players().await.expect("Failed to get players");

        let players: Vec<fpl_players::FplPlayer> = json_players
            .iter()
            .map(|player| {
                fpl_conversions::convert_player(player).expect("Failed to convert player")
            })
            .collect();

        // Get the indices of players we want to review
        let player_indices = vec![0, 10, 25, 50];
        for player_idx in player_indices {
            let player = &players[player_idx];
            let calculated_stats = player.get_stats_per_90();

            assert_relative_eq!(player.stats_per_90, calculated_stats, epsilon=1e-2);
        }
    }

    #[tokio::test]
    async fn test_get_teams() {
        let json_teams = get_teams().await.expect("Failed to get teams");

        let teams: Vec<fpl_teams::FplTeam> = json_teams
            .iter()
            .map(|team| fpl_conversions::convert_team(team).expect("Failed to convert team"))
            .collect();

        assert_eq!(teams.len(), 20);
        assert_eq!(teams[0].name, "Arsenal");
        assert_eq!(teams[0].short_name, "ARS");

        assert_eq!(teams[0].table_data.played, 0);
    }
}
