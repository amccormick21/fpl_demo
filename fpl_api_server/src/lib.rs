mod fpl_players {
    use std::collections::HashMap;

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

    pub struct FplPlayerList {
        pub player_list: HashMap<u32, FplPlayer>,
    }

    impl FplPlayerList {
        // Initialise the collection
        pub fn new() -> Self {
            FplPlayerList {
                player_list: HashMap::new(),
            }
        }

        // Get a specific player
        pub fn get_player(&self, player_id: &u32) -> Option<&FplPlayer> {
            self.player_list.get(&player_id)
        }

        pub fn get_player_by_name(&self, player_name: String) -> Option<&FplPlayer> {
            if let Some((_, player_with_name)) = self
                .player_list
                .iter()
                .find(|(_, player)| player.name.second_name == player_name)
            {
                Some(player_with_name)
            } else {
                None
            }
        }

        pub fn add_player(&mut self, player_id: u32, player: FplPlayer) {
            self.player_list.insert(player_id, player);
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
        pub position: Position,
    }
}

mod fpl_match_stats {

    use crate::{
        fpl_fixtures::player_points, fpl_players::{self, FplPlayer, FplPlayerList}, fpl_positions
    };
    use std::{collections::HashMap, hash::Hash};

    #[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
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
        pub fn from(identifier: &str) -> Result<MatchStatistic, String> {
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
                _ => Err(format!("Could not convert statistic")),
            }
        }
    }

    type PlayerToStatisticMap = HashMap<u32, MatchStatisticValueMap>;
    type PlayerToStatisticIter =
        std::collections::hash_map::Iter<'static, u32, MatchStatisticValueMap>;

    pub type MatchStatisticValueMap = HashMap<MatchStatistic, i32>;
    pub type MatchStatisticValueIter =
        std::collections::hash_map::Iter<'static, MatchStatistic, i32>;

    #[derive(Debug)]
    pub struct MatchStatisticMap {
        match_stats: HashMap<u32, MatchStatisticValueMap>,
    }

    #[derive(Debug)]
    struct PlayerPointPair<'a> {
        pub player: &'a fpl_players::FplPlayer,
        pub bonus_points: i32,
    }

    impl MatchStatisticMap {
        pub fn new() -> Self {
            MatchStatisticMap {
                match_stats: HashMap::new(),
            }
        }

        // Function to extract the bonus points from the match stat and form a simplified structure
        fn get_bonus_points_per_player<'a>(
            player_id: &u32,
            stat_player: &MatchStatisticValueMap,
            player_list: &'a fpl_players::FplPlayerList,
        ) -> PlayerPointPair<'a> {
            PlayerPointPair {
                player: player_list.get_player(player_id).unwrap(),
                bonus_points: stat_player
                    .get(&MatchStatistic::Bps)
                    .or(Some(&0))
                    .cloned()
                    .unwrap(),
            }
        }

        fn rank_players_by_bonus_points<'a>(
            player_points_in_match: Vec<PlayerPointPair<'a>>,
        ) -> Vec<(&'a FplPlayer, usize)> {
            // Rank the points
            let mut rankings: Vec<(&'a FplPlayer, usize)> = Vec::new();
            let mut current_rank: usize = 1;
            let mut previous_points = None;

            for (index, player) in player_points_in_match.iter().enumerate() {
                if Some(player.bonus_points) != previous_points {
                    current_rank = index + 1;
                }
                rankings.push((player.player, current_rank));
                previous_points = Some(player.bonus_points);
            }

            rankings
        }

        fn player_points_from_ranking<'a>(
            player_rankings: Vec<(&'a FplPlayer, usize)>,
        ) -> Vec<(&'a FplPlayer, i32)> {
            player_rankings
                .into_iter()
                .map(|(player, rank)| -> (&FplPlayer, i32) {
                    (player, bonus_points_rank_to_points(rank))
                })
                .collect()
        }

        pub fn get_player_points_from_stats<'a>(
            &mut self,
            player_list: &'a fpl_players::FplPlayerList,
        ) -> Vec<(&'a FplPlayer, i32)> {
            // BONUS POINTS
            // Extract a list of players and bonus points
            let mut player_bonus_points_in_match: Vec<PlayerPointPair> = self
                .match_stats
                .iter()
                .map(|stats: (&u32, &HashMap<MatchStatistic, i32>)| {
                    Self::get_bonus_points_per_player(stats.0, stats.1, player_list)
                })
                .collect();

            // Sort the players by the most points
            player_bonus_points_in_match.sort_by_key(|p| p.bonus_points);

            let rankings = Self::rank_players_by_bonus_points(player_bonus_points_in_match);

            let bonus_points = Self::player_points_from_ranking(rankings);

            let set_stat_value = |&(player, points): &(&FplPlayer, i32)| {
                self.match_stats.get_mut(&player.id).unwrap().insert(MatchStatistic::Bonus, points);
            };
            bonus_points.iter().for_each(set_stat_value);

            // FINISHED THE BONUS POINTS

            // Now sum up the FPL points
            let sum_fpl_points =
                |(&player_id, match_stats): (&u32, &MatchStatisticValueMap)| -> (&'a FplPlayer, i32) {
                    let player = player_list.get_player(&player_id).unwrap();
                    (player, player_points(&match_stats, &player))
                };
            self.match_stats.iter().map(sum_fpl_points).collect()
        }

        // Get a list of all of the players in the match
        fn get_players_in_stats<'a>(&self, players: &'a FplPlayerList) -> Vec<&'a FplPlayer> {
            // Find all of the players in the hash map indices
            let match_players: Vec<&u32> = self.match_stats.keys().collect();

            match_players
                .into_iter()
                .map(|player_id| players.get_player(player_id).unwrap())
                .collect()
        }

        pub fn add_statistic(
            &mut self,
            element: u32,
            identifier: MatchStatistic,
            value: i32,
        ) -> Option<i32> {
            self.match_stats
                .entry(element)
                .or_default()
                .insert(identifier, value)
        }
    }

    pub fn bonus_points_rank_to_points(rank: usize) -> i32 {
        match rank {
            1 => 3,
            2 => 2,
            3 => 1,
            _ => 0,
        }
    }

    // TODO: unit testing for these arrays

    pub fn points_multiplier(
        match_stat: &MatchStatistic,
        player_position: &fpl_positions::Position,
    ) -> i32 {
        match match_stat {
            MatchStatistic::GoalsScored => match player_position {
                fpl_positions::Position::FWD => 4,
                fpl_positions::Position::MID => 5,
                fpl_positions::Position::DEF | fpl_positions::Position::GK => 6,
            },
            MatchStatistic::Assists => 3,
            MatchStatistic::Bps => 0,
            MatchStatistic::Bonus => 1,
            MatchStatistic::OwnGoals => -2,
            MatchStatistic::YellowCards => -1,
            MatchStatistic::RedCards => -3,
            MatchStatistic::Saves => 3,
            MatchStatistic::PenaltiesMissed => -2,
            MatchStatistic::PenaltiesSaved => 5,
            MatchStatistic::Minutes => 1,
        }
    }

    pub fn points_calculator(match_stat: &MatchStatistic, stat_value: i32) -> i32 {
        match match_stat {
            MatchStatistic::GoalsScored => stat_value,
            MatchStatistic::Assists => stat_value,
            MatchStatistic::Bps => 0, // You can't get points for Bps
            MatchStatistic::Bonus => stat_value,
            MatchStatistic::OwnGoals => stat_value,
            MatchStatistic::YellowCards => stat_value,
            MatchStatistic::RedCards => stat_value,
            MatchStatistic::Saves => stat_value / 3,
            MatchStatistic::PenaltiesMissed => stat_value,
            MatchStatistic::PenaltiesSaved => stat_value,
            MatchStatistic::Minutes => match stat_value {
                minutes if minutes > 60 => 2,
                minutes if minutes > 0 => 1,
                _ => 0,
            },
        }
    }
}

mod fpl_fixtures {
    use chrono::{DateTime, Utc};

    use crate::{
        fpl_match_stats::{
            points_calculator, points_multiplier, MatchStatistic, MatchStatisticMap,
            MatchStatisticValueMap,
        },
        fpl_players::{self, FplPlayer},
        fpl_teams,
    };

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
        pub stats: MatchStatisticMap,
    }

    impl Match {
        pub fn home_team(self, teams: &Vec<fpl_teams::FplTeam>) -> String {
            if let Some(team) = teams.iter().find(|&t| t.id == self.home_team_id) {
                String::from(&team.name)
            } else {
                "N/A".to_string()
            }
        }

        pub fn away_team(self, teams: &Vec<fpl_teams::FplTeam>) -> String {
            if let Some(team) = teams.iter().find(|&t| t.id == self.away_team_id) {
                String::from(&team.name)
            } else {
                "N/A".to_string()
            }
        }

        fn points_for_match<'a>(
            &mut self,
            player_list: &'a fpl_players::FplPlayerList,
        ) -> Vec<(&'a FplPlayer, i32)> {
            self.stats.get_player_points_from_stats(player_list)
        }
    }

    pub fn player_points(stats_list: &MatchStatisticValueMap, player: &fpl_players::FplPlayer) -> i32 {
        let get_stat_points = |(stat, stat_value): (&MatchStatistic, &i32)| -> i32 {
            points_multiplier(&stat, &player.position) * points_calculator(&stat, *stat_value)
        };

        stats_list.iter().map(get_stat_points).sum()
    }

    pub struct MatchList {
       pub match_list: Vec<Match>
    }

    impl MatchList {

        pub fn new() -> Self {
            MatchList {
                match_list: Vec::new(),
            }
        }

        pub fn add_fixture(&mut self, fixture: Match) {
            self.match_list.push(fixture);
        }
    }
}

mod fpl_conversions {
    use chrono::DateTime;

    use crate::fpl_fixtures;
    use crate::fpl_fixtures::MatchScore;
    use crate::fpl_match_stats::MatchStatistic;
    use crate::fpl_match_stats::MatchStatisticMap;
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
            position: fpl_positions::Position::try_from(api_position.id - 1)
                .expect("Unexpected position ID, could not map to a position GK/DEF/MID/FWD"),
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
            position: fpl_positions::Position::try_from(api_player.element_type - 1)
                .expect("Could not convert position"),
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

    pub fn convert_player_list(
        api_player_list: &Vec<fpl_data::fpl_data::FplApiPlayer>,
    ) -> Result<fpl_players::FplPlayerList, &str> {
        let mut player_list = fpl_players::FplPlayerList::new();

        api_player_list.into_iter().for_each(|api_player| {
            player_list.add_player(api_player.id, convert_player(api_player))
        });

        Ok(player_list)
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
    ) -> Result<fpl_fixtures::Match, String> {
        let match_score = if let (Some(home_score), Some(away_score)) =
            (api_fixture.team_h_score, api_fixture.team_a_score)
        {
            Some(MatchScore {
                home: home_score,
                away: away_score,
            })
        } else {
            None
        };

        // The array "match_stats" is a map of player -> statistics
        let mut match_stats = MatchStatisticMap::new();

        api_fixture.stats.iter().for_each(|stat| {
            stat.h.iter().for_each(|player_id| {
                match_stats.add_statistic(
                    player_id.element,
                    MatchStatistic::from(&stat.identifier).unwrap(),
                    player_id.value,
                );
            });
            stat.a.iter().for_each(|player_id| {
                match_stats.add_statistic(
                    player_id.element,
                    MatchStatistic::from(&stat.identifier).unwrap(),
                    player_id.value,
                );
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

    pub fn convert_fixture_list(
        api_fixture_list: &Vec<fpl_data::fpl_data::FplApiFixture>,
    ) -> Result<fpl_fixtures::MatchList, &str> {
        let mut fixture_list = fpl_fixtures::MatchList::new();

        api_fixture_list.into_iter().for_each(|api_fixture| {
            fixture_list.add_fixture(convert_fixture(api_fixture).unwrap())
        });

        Ok(fixture_list)
    }
}

#[cfg(test)]
mod tests {

    #[cfg(test)]
    mod test_statistics {

        use std::{collections::HashMap};

        use crate::{fpl_conversions, fpl_match_stats::{self, MatchStatisticMap}, fpl_players::FplPlayer};
        use fpl_data::fpl_data;

        #[tokio::test]
        async fn test_calculate_points() {
            let api_players = fpl_data::get_players()
                .await
                .expect("Failed to get players");

            let player_list = fpl_conversions::convert_player_list(&api_players).unwrap();

            let player = player_list
                .get_player_by_name(String::from("Haaland"))
                .unwrap();

            let mut match_statistic = fpl_match_stats::MatchStatisticMap::new();
            match_statistic.add_statistic(
                player.id,
                fpl_match_stats::MatchStatistic::GoalsScored,
                1,
            );
            match_statistic.add_statistic(player.id, fpl_match_stats::MatchStatistic::Assists, 2);
            match_statistic.add_statistic(player.id, fpl_match_stats::MatchStatistic::Minutes, 72);

            let points_from_stats = match_statistic.get_player_points_from_stats(&player_list);

            let first_player_points = points_from_stats[0];

            assert_eq!(first_player_points.0.id, player.id);
            assert_eq!(first_player_points.1, 15);
        }

        #[tokio::test]
        async fn validate_points_calculation_in_matches() {
            
            let api_players = fpl_data::get_players()
                .await
                .expect("Failed to get players");

            let player_list = fpl_conversions::convert_player_list(&api_players).unwrap();

            let api_games = fpl_data::get_fixtures()
                .await
                .expect("Failed to get matches");

            let match_list = fpl_conversions::convert_fixture_list(&api_games).unwrap();

            // Perform our own calculation to get the points in the match
            // Let's figure this out later
            let match_stats: Vec<MatchStatisticMap> = match_l ist.match_list.into_iter().map(|fixture| fixture.stats).collect();
            let calculated_points: Vec<Vec<(&FplPlayer, i32)>> = match_stats.into_iter().map(|mut stats| stats.get_player_points_from_stats(&player_list)).collect();
            
            // Now we need to add this up over the range of games
            let mut player_to_points_map: HashMap<u32, i32> = HashMap::new();

            let mut assign_player_points = |player: &FplPlayer, points: i32| {
                let player_node = player_to_points_map.entry(player.id).or_default();
                *player_node += points;
            };

            let assign_game_points = |points_in_game: Vec<(&FplPlayer, i32)>| {
                points_in_game.into_iter().for_each(|(player, points)| assign_player_points(player, points));
            };
            
            calculated_points.into_iter().for_each(assign_game_points);

            // player_to_points_map should now contain the right points for the season so far
            let check_points = |(player_id, points): (&u32, &i32)| {
                let player = player_list.get_player(&player_id).unwrap();
                println!("Player {} has {} points, and we think he's got {} points", player.name.display_name, player.points_record.total_points, *points);
                // assert_eq!(player_list.get_player(&player_id).unwrap().points_record.total_points, *points);
            };
            player_to_points_map.iter().for_each(check_points);

        }
    }

    #[cfg(test)]
    mod test_conversions {

        use approx::assert_relative_eq;
        use fpl_data::fpl_data;

        use crate::{fpl_conversions, fpl_fixtures, fpl_positions, fpl_teams};

        #[tokio::test]
        async fn test_get_events() {
            let events = fpl_data::get_events().await.expect("Failed to get events");

            let _ = events.first().unwrap();
        }

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

            let players = fpl_conversions::convert_player_list(&api_players).unwrap();

            assert_eq!(players.get_player(&3).unwrap().name.display_name, "Gabriel");
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
            let api_fixtures = fpl_data::get_fixtures()
                .await
                .expect("Failed to get fixtures");

            let fixtures: Vec<fpl_fixtures::Match> = api_fixtures
                .iter()
                .map(|api_fixture| {
                    fpl_conversions::convert_fixture(api_fixture)
                        .expect("Failed to convert fixture")
                })
                .collect();

            assert_eq!(fixtures.len(), 19 * 20);
            assert!(fixtures[0].finished);
            assert_eq!(fixtures[0].home_team_id, 14);
        }
    }
}
