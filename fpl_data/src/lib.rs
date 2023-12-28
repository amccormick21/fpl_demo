pub mod fpl_data {
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize, Debug)]
    pub struct FplTeamStrength {
        pub overall_home: u64,
        pub overall_away: u64,
        pub attack_home: u64,
        pub attack_away: u64,
        pub defence_home: u64,
        pub defence_away: u64,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct FplTeam {
        pub id: u64,
        pub code: u64,
        pub name: String,
        pub short_name: String,
        // pub team_division: u32,
        // pub form: i32,
        pub pulse_id: u32,
        pub strength: u32,
        pub played: u32,
        pub win: u32,
        pub draw: u32,
        pub loss: u32,
        pub points: u32,
        pub position: u32,
        pub strengths: FplTeamStrength,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
