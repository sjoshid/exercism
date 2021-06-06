use std::collections::HashSet;

use crate::Team;

pub const WIN_POINTS: u64 = 3;
pub const DRAW_POINTS: u64 = 1;

pub struct League {
    teams: HashSet<Team>,
}

impl League {
    pub fn new() -> Self {
        League {
            teams: HashSet::new(),
        }
    }

    pub fn update_winning_team(&mut self, team_name: &str) {
        if let Some(t) = self.teams.get(team_name) {
            let mut t = t.clone();
            t.matches_won += 1;
            t.matches_played += 1;
            t.points += WIN_POINTS;
            self.teams.replace(t);
        } else {
            let mut t = Team::new(team_name);
            t.matches_won += 1;
            t.matches_played += 1;
            t.points += WIN_POINTS;
            self.teams.insert(t);
        }
    }

    pub fn update_losing_team(&mut self, team_name: &str) {
        if let Some(t) = self.teams.get(team_name) {
            let mut t = t.clone();
            t.matches_played += 1;
            t.matches_lost += 1;
            self.teams.replace(t);
        } else {
            let mut t = Team::new(team_name);
            t.matches_played += 1;
            t.matches_lost += 1;
            self.teams.insert(t);
        }
    }

    pub fn update_draw_teams(&mut self, first_team: &str, second_team: &str) {
        self.update_draw(first_team);
        self.update_draw(second_team);
    }

    fn update_draw(&mut self, team_name: &str) {
        if let Some(t) = self.teams.get(team_name) {
            let mut t = t.clone();
            t.matches_played += 1;
            t.matches_drawn += 1;
            t.points += DRAW_POINTS;
            self.teams.replace(t);
        } else {
            let mut t = Team::new(team_name);
            t.matches_played += 1;
            t.matches_drawn += 1;
            t.points += DRAW_POINTS;
            self.teams.insert(t);
        }
    }
}

impl From<League> for String {
    fn from(mut league: League) -> Self {
        let mut league = league.teams.drain().collect::<Vec<Team>>();
        league.sort_by(|t1, t2| t2.points.cmp(&t1.points).then(t1.name.cmp(&t2.name)));
        println!("Sorted league {:?}", league);
        String::from("dummy")
    }
}
