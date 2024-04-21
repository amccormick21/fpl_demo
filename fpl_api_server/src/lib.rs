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
    pub struct FplPlayerName {
        pub first_name: String,
        pub second_name: String,
        pub display_name: String,
    }

    pub struct FplPlayerStats {
        pub goals_scored: u32,
        pub assists: u32,
        pub clean_sheets: u32,
        pub expected_goals: f32,
        pub expected_assists: f32,
        pub expected_goal_involvements: f32,
        pub expected_clean_sheets: f32,
    }

    pub struct FplPlayer {
        pub id: u32,
        pub name: FplPlayerName,
        pub stats: FplPlayerStats,
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
    use crate::fpl_teams;
    use crate::fpl_players;

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
                goals_scored: api_player.goals_scored,
                assists: api_player.assists,
                clean_sheets: api_player.clean_sheets,
                expected_goals: api_player.expected_goals.parse().expect("Expected goals was not a number"),
                expected_assists: api_player.expected_assists.parse().expect("Expected assists was not a number"),
                expected_goal_involvements: api_player.expected_goal_involvements.parse().expect("Expected goal involvements was not a number"),
                expected_clean_sheets: api_player.expected_goals_conceded.parse().expect("Expected goals conceded was not a number"),
            },
        })
    }


    pub fn convert_team(
        json_team: &serde_json::Value,
    ) -> Result<fpl_teams::FplTeam, &str> {

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
            .map(|player| fpl_conversions::convert_player(player).expect("Failed to convert player"))
            .collect();

        assert_eq!(players[3].name.display_name, "Fábio Vieira");
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
