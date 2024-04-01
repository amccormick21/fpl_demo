use reqwest;

pub async fn get_all_data() -> Result<serde_json::Value, reqwest::Error> {
    let request_url = format!("https://fantasy.premierleague.com/api/bootstrap-static/");

    println!("{}", request_url);
    let response = reqwest::get(&request_url).await?;
    println!("{}", response.status());

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

mod fpl_conversions {

    pub fn convert_team(
        json_team: &serde_json::Value,
    ) -> Result<fpl_data::fpl_data::FplTeam, &str> {
        let team: fpl_data::fpl_data::FplTeam =
            serde_json::from_value(json_team.clone()).expect("Failed to convert team");
        Ok(team)
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
    async fn test_get_teams() {
        let json_teams = get_teams().await.expect("Failed to get teams");

        let teams: Vec<fpl_data::fpl_data::FplTeam> = json_teams
            .iter()
            .map(|team| fpl_conversions::convert_team(team).expect("Failed to convert team"))
            .collect();

        assert_eq!(teams.len(), 20);
        assert_eq!(teams[0].name, "Arsenal");
        assert_eq!(teams[0].short_name, "ARS");
    }
}
