mod fpl_players {
    use crate::fpl_positions;

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
            if nineties == 0.0 {
                FplPlayerStatsPer90 {
                    starts: 0.0,
                    goals: 0.0,
                    goals_conceded: 0.0,
                    clean_sheets: 0.0,
                }
            } else {
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
            f64::abs_diff_eq(&self.starts, &other.starts, epsilon)
                && f64::abs_diff_eq(&self.goals, &other.goals, epsilon)
                && f64::abs_diff_eq(&self.goals_conceded, &other.goals_conceded, epsilon)
                && f64::abs_diff_eq(&self.clean_sheets, &other.clean_sheets, epsilon)
        }
    }

    impl approx::RelativeEq for FplPlayerStatsPer90 {
        fn default_max_relative() -> Self::Epsilon {
            f64::default_max_relative()
        }

        fn relative_eq(
            &self,
            other: &Self,
            epsilon: Self::Epsilon,
            max_relative: Self::Epsilon,
        ) -> bool {
            f64::relative_eq(&self.starts, &other.starts, epsilon, max_relative)
                && f64::relative_eq(&self.goals, &other.goals, epsilon, max_relative)
                && f64::relative_eq(
                    &self.goals_conceded,
                    &other.goals_conceded,
                    epsilon,
                    max_relative,
                )
                && f64::relative_eq(
                    &self.clean_sheets,
                    &other.clean_sheets,
                    epsilon,
                    max_relative,
                )
        }
    }

    impl approx::UlpsEq for FplPlayerStatsPer90 {
        fn default_max_ulps() -> u32 {
            f64::default_max_ulps()
        }

        fn ulps_eq(&self, other: &Self, epsilon: Self::Epsilon, max_ulps: u32) -> bool {
            f64::ulps_eq(&self.starts, &other.starts, epsilon, max_ulps)
                && f64::ulps_eq(&self.goals, &other.goals, epsilon, max_ulps)
                && f64::ulps_eq(
                    &self.goals_conceded,
                    &other.goals_conceded,
                    epsilon,
                    max_ulps,
                )
                && f64::ulps_eq(&self.clean_sheets, &other.clean_sheets, epsilon, max_ulps)
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
        pub position: fpl_positions::Position,
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

mod fpl_positions {

    use std::convert::TryFrom;

    #[derive(Debug, PartialEq)]
    pub enum Position {
        GK,
        DEF,
        MID,
        FWD,
    }

    impl TryFrom<u32> for Position {
        type Error = ();
    
        fn try_from(v: u32) -> Result<Self, Self::Error> {
            match v {
                x if x == Position::GK as u32 => Ok(Position::GK),
                x if x == Position::DEF as u32 => Ok(Position::DEF),
                x if x == Position::MID as u32 => Ok(Position::MID),
                x if x == Position::FWD as u32 => Ok(Position::FWD),
                _ => Err(()),
            }
        }
    }

    #[derive(Debug, PartialEq)]
    pub struct FplPosition {
        pub id: u32,
        pub squad_select: u32,
        pub squad_min_play: u32,
        pub squad_max_play: u32,
        pub element_count: u32,
        pub position: Position
    }
}

mod fpl_match_stats {

    use crate::fpl_positions;

    #[derive(Debug)]
    pub enum MatchStatistic {
        GoalsScored,
        Assists,
        OwnGoals,
        PenaltiesSaved,
        PenaltiesMissed,
        YellowCards,
        RedCards,
        Saves,
        Bonus,
        Bps,
        Minutes,
    }

    impl MatchStatistic {
        pub fn from(identifier: &str) -> Result<MatchStatistic, &'static str> {
            
            match identifier {
                "goals_scored" => Ok(MatchStatistic::GoalsScored),
                "assists" => Ok(MatchStatistic::Assists),
                "own_goals" => Ok(MatchStatistic::OwnGoals),
                "penalties_saved" => Ok(MatchStatistic::PenaltiesSaved),
                "penalties_missed" => Ok(MatchStatistic::PenaltiesMissed),
                "yellow_cards" => Ok(MatchStatistic::YellowCards),
                "red_cards" => Ok(MatchStatistic::RedCards),
                "saves" => Ok(MatchStatistic::Saves),
                "bonus" => Ok(MatchStatistic::Bonus),
                "bps" => Ok(MatchStatistic::Bps),
                "minutes" => Ok(MatchStatistic::Minutes),
                _ => Err("Could not convert statistic"),
            }
        }
    }

    // TODO: unit testing for these arrays

    pub fn points_multiplier(match_stat: &MatchStatistic, player_position: &fpl_positions::Position) -> i32
    {
        match match_stat {
            MatchStatistic::GoalsScored => match player_position {
                fpl_positions::Position::FWD => 4,
                fpl_positions::Position::MID => 5,
                fpl_positions::Position::DEF | fpl_positions::Position::GK => 6,
            },
            MatchStatistic::Assists => 3,
            MatchStatistic::Bps => 1,
            MatchStatistic::Bonus => 0, // TODO: check the difference between Bps and Bonus
            MatchStatistic::OwnGoals => -2,
            MatchStatistic::YellowCards => -1,
            MatchStatistic::RedCards => -3,
            MatchStatistic::Saves => 3,
            MatchStatistic::PenaltiesMissed => -2,
            MatchStatistic::PenaltiesSaved => 5,
            MatchStatistic::Minutes => 1, 
        }
    } 

    pub fn points_calculator(match_stat: &MatchStatistic, stat_value: i32) -> i32
    {
        match match_stat {
            MatchStatistic::GoalsScored => stat_value,
            MatchStatistic::Assists => stat_value,
            MatchStatistic::Bps => stat_value,
            MatchStatistic::Bonus => stat_value, // TODO: check the difference between Bps and Bonus
            MatchStatistic::OwnGoals => stat_value,
            MatchStatistic::YellowCards => stat_value,
            MatchStatistic::RedCards => stat_value,
            MatchStatistic::Saves => stat_value / 3,
            MatchStatistic::PenaltiesMissed => stat_value,
            MatchStatistic::PenaltiesSaved => stat_value,
            MatchStatistic::Minutes => match stat_value {
                minutes if minutes > 0 => 1,
                minutes if minutes > 60 => 2,
                _ => 0
            }
        }
    }

    #[derive(Debug)]
    pub struct MatchStatisticData {
        pub statistic: MatchStatistic,
        pub value: i32
    }

    impl MatchStatisticData {
        pub fn from(identifier: &str, data: i32) -> Result<MatchStatisticData, &'static str> {
            
            let stat_result = MatchStatistic::from(identifier);
            if let Ok(match_statistic) = stat_result {
                Ok(MatchStatisticData {
                    statistic: match_statistic,
                    value: data
                })
            }
            else {
                Err(stat_result.unwrap_err())
            }
        }
    }
}

mod fpl_fixtures {
    use chrono::{DateTime, Utc};
    use std::collections::HashMap;

    use crate::{fpl_match_stats::{self, points_calculator, points_multiplier, MatchStatisticData}, fpl_players, fpl_teams};

    #[derive(Debug)]
    pub struct MatchScore {
        pub home: u32,
        pub away: u32,
        // TODO: derive result, points, etc.
    }

    #[derive(Debug)]
    pub struct Match {
        pub code: u32,
        pub event: u32,
        pub finished: bool,
        pub finished_provisional: bool,
        pub id: u32,
        pub kickoff_time: DateTime<Utc>,
        pub minutes: u32,
        pub provisional_start_time: bool,
        pub started: bool,
        pub home_team_id: u64,
        pub away_team_id: u64,
        pub score: Option<MatchScore>, // If the game has not been played then there is no score
        pub stats: HashMap<u32, Vec<fpl_match_stats::MatchStatisticData>>,
    }

    impl Match {
        pub fn home_team(self, teams: &Vec<fpl_teams::FplTeam>) -> String {
            if let Some(team) = teams.iter().find(|&t| {t.id == self.home_team_id} ) {
                String::from(&team.name)
            }
            else {
                "N/A".to_string()
            }
        }

        pub fn away_team(self, teams: &Vec<fpl_teams::FplTeam>) -> String {
            if let Some(team) = teams.iter().find(|&t| {t.id == self.away_team_id} ) {
                String::from(&team.name)
            }
            else {
                "N/A".to_string()
            }
        }
    }

    fn player_points(stats_list: &Vec<MatchStatisticData>, player: &fpl_players::FplPlayer) -> i32 {

        let get_stat_points = |stat: &MatchStatisticData| -> i32 { points_multiplier(&stat.statistic, &player.position) * points_calculator(&stat.statistic, stat.value)
        };

        stats_list.iter().map(get_stat_points).sum()
    }
}

mod fpl_conversions {
    use chrono::DateTime;
    use std::collections::HashMap;

    use crate::fpl_fixtures;
    use crate::fpl_fixtures::MatchScore;
    use crate::fpl_match_stats::MatchStatisticData;
    use crate::fpl_players;
    use crate::fpl_positions;
    use crate::fpl_teams;

    pub fn convert_position(
        api_position: &fpl_data::fpl_data::FplApiPosition,
    ) -> Result<fpl_positions::FplPosition, &'static str> {
        let fpl_position = fpl_positions::FplPosition {
            id: api_position.id,
            squad_select: api_position.squad_select,
            squad_min_play: api_position.squad_min_play,
            squad_max_play: api_position.squad_max_play,
            element_count: api_position.element_count,
            position: fpl_positions::Position::try_from(api_position.id - 1).expect("Unexpected position ID, could not map to a position GK/DEF/MID/FWD"),
        };

        Ok(fpl_position)
    }

    pub fn convert_player(api_player: &fpl_data::fpl_data::FplApiPlayer) -> fpl_players::FplPlayer {
        fpl_players::FplPlayer {
            id: api_player.id,
            name: fpl_players::FplPlayerName {
                first_name: api_player.first_name.clone(),
                second_name: api_player.second_name.clone(),
                display_name: api_player.web_name.clone(),
            },
            position: fpl_positions::Position::try_from(api_player.element_type-1).expect("Could not convert position"),
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
        }
    }

    pub fn convert_team(
        api_team: &fpl_data::fpl_data::FplApiTeam,
    ) -> Result<fpl_teams::FplTeam, &str> {
        Ok(fpl_teams::FplTeam {
            id: api_team.id,
            name: api_team.name.clone(),
            short_name: api_team.short_name.clone(),
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

    pub fn convert_fixture(
        api_fixture: &fpl_data::fpl_data::FplApiFixture,
    ) -> Result<fpl_fixtures::Match, &str> {
        
        let match_score = if let (Some(home_score), Some(away_score)) = (api_fixture.team_h_score, api_fixture.team_a_score) {
            Some(MatchScore {
                home: home_score,
                away: away_score
            })
        } else {
            None
        };

        let mut match_stats: HashMap<u32, Vec<MatchStatisticData>> = HashMap::new();

        api_fixture.stats.iter().for_each(|stat| {
            stat.h.iter().for_each(|player_id| {
                match_stats.entry(player_id.element).or_default().push(
                    MatchStatisticData::from(&stat.identifier, player_id.value).expect("Could not convert statistic for home player")
                )
            });
            stat.a.iter().for_each(|player_id| {
                match_stats.entry(player_id.element).or_default().push(
                    MatchStatisticData::from(&stat.identifier, player_id.value).expect("Could not convert statistic for away player")
                )
            });
        });
        
        Ok(fpl_fixtures::Match {
            code: api_fixture.code,
            event: api_fixture.event,
            finished: api_fixture.finished,
            finished_provisional: api_fixture.finished_provisional,
            id: api_fixture.id,
            kickoff_time: DateTime::parse_from_rfc3339(&api_fixture.kickoff_time)
                .unwrap()
                .into(),
            minutes: api_fixture.minutes,
            provisional_start_time: api_fixture.provisional_start_time,
            started: api_fixture.started,
            home_team_id: api_fixture.team_h,
            away_team_id: api_fixture.team_a,
            score: match_score,
            stats: match_stats,
        })
    }
}

#[cfg(test)]
mod tests {

    use approx::assert_relative_eq;
    use fpl_data::fpl_data;

    use super::*;

    #[tokio::test]
    async fn test_get_positions() {
        let api_positions = fpl_data::get_positions()
            .await
            .expect("Failed to get positions");

        let positions: Vec<fpl_positions::FplPosition> = api_positions
            .iter()
            .map(|position| {
                fpl_conversions::convert_position(position).expect("Failed to convert position")
            })
            .collect();

        assert_eq!(positions[0].id, 1);
        assert_eq!(positions[1].id, 2);
    }

    #[tokio::test]
    async fn test_get_players() {
        let api_players = fpl_data::get_players()
            .await
            .expect("Failed to get players");

        let players: Vec<fpl_players::FplPlayer> = api_players
            .iter()
            .map(fpl_conversions::convert_player)
            .collect();

        assert_eq!(players[3].name.display_name, "Fábio Vieira");
    }

    #[tokio::test]
    async fn test_stats_per_90() {
        let api_players = fpl_data::get_players()
            .await
            .expect("Failed to get players");
        let player_indices = vec![0, 10, 25, 50];

        // Get the indices of players we want to review
        for player_idx in player_indices {
            let api_player = &api_players[player_idx];

            let player = fpl_conversions::convert_player(api_player);
            let calculated_stats = player.get_stats_per_90();

            assert_relative_eq!(player.stats_per_90, calculated_stats, epsilon = 1e-2);
        }
    }

    #[tokio::test]
    async fn test_get_teams() {
        let json_teams = fpl_data::get_teams().await.expect("Failed to get teams");

        let teams: Vec<fpl_teams::FplTeam> = json_teams
            .iter()
            .map(|team| fpl_conversions::convert_team(team).expect("Failed to convert team"))
            .collect();

        assert_eq!(teams.len(), 20);
        assert_eq!(teams[0].name, "Arsenal");
        assert_eq!(teams[0].short_name, "ARS");

        assert_eq!(teams[0].table_data.played, 0);
    }

    #[tokio::test]
    async fn test_get_fixtures() {
        let api_fixtures = fpl_data::get_fixtures().await.expect("Failed to get fixtures");

        let fixtures: Vec<fpl_fixtures::Match> = api_fixtures
            .iter()
            .map(|api_fixture| fpl_conversions::convert_fixture(api_fixture).expect("Failed to convert fixture"))
            .collect();

        assert_eq!(fixtures.len(), 19 * 20);
        assert!(fixtures[0].finished);
        assert_eq!(fixtures[0].home_team_id, 6);
    }
}
