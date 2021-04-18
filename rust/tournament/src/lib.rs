use std::borrow::Borrow;
use std::collections::HashSet;
use std::hash::{Hash, Hasher};

pub const WIN_POINTS: u8 = 3;
pub const DRAW_POINTS: u8 = 1;

#[derive(Eq, Clone, Debug)]
pub struct Team {
    name: String,
    matches_played: u64,
    matches_won: u64,
    matches_drawn: u64,
    matches_lost: u64,
    points: u64,
}

impl Hash for Team {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

impl PartialEq for Team {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Borrow<str> for Team {
    fn borrow(&self) -> &str {
        self.name.as_str()
    }
}

pub fn tally(match_results: &str) -> String {
    let mut teams: HashSet<Team> = HashSet::new();
    for l in match_results.split("\n").into_iter() {
        //println!("{}", l);
        let mut game = l.split(";");
        let ft = game.next().unwrap_or("First team missing");
        let st = game.next().unwrap_or("Second team missing");
        let outcome = game.next().unwrap_or("Outcome missing");

        let (ft, st, outcome) = match outcome {
            "loss" => (st, ft, outcome),
            "draw" | "win" => (ft, st, outcome),
            _ => {
                panic!("Invalid outcome.")
            }
        };

        match outcome {
            "win" | "loss" => {
                if let Some(t) = teams.get(ft) {
                    //println!("Updating {}", ft);
                    let mut t = t.clone();
                    t.matches_played += 1;
                    t.matches_won += 1;
                    t.points += 3;
                    teams.replace(t);
                    //println!("{:?}", teams.iter());
                } else {
                    //println!("Inserting {}", ft);
                    teams.insert(Team {
                        name: String::from(ft),
                        matches_played: 1,
                        matches_won: 1,
                        matches_drawn: 0,
                        matches_lost: 0,
                        points: 3,
                    });
                }

                if let Some(t) = teams.get(st) {
                    //println!("Updating {}", st);
                    let mut t = t.clone();
                    t.matches_played += 1;
                    t.matches_lost += 1;
                    teams.replace(t);
                    //println!("{:?}", teams.iter());
                } else {
                    //println!("Inserting {}", st);
                    teams.insert(Team {
                        name: String::from(st),
                        matches_played: 1,
                        matches_won: 0,
                        matches_drawn: 0,
                        matches_lost: 1,
                        points: 0,
                    });
                }
            }
            "draw" => {
                if let Some(t) = teams.get(ft) {
                    let mut t = t.clone();
                    t.matches_played += 1;
                    t.matches_drawn += 1;
                    t.points += 1;

                    teams.insert(t);
                } else {
                    teams.insert(Team {
                        name: String::from(ft),
                        matches_played: 1,
                        matches_won: 0,
                        matches_drawn: 1,
                        matches_lost: 0,
                        points: 1,
                    });
                }

                if let Some(t) = teams.get(st) {
                    let mut t = t.clone();
                    t.matches_played += 1;
                    t.matches_drawn += 1;
                    t.points += 1;

                    teams.insert(t);
                } else {
                    teams.insert(Team {
                        name: String::from(st),
                        matches_played: 1,
                        matches_won: 0,
                        matches_drawn: 1,
                        matches_lost: 0,
                        points: 1,
                    });
                }
            }
            _ => {
                panic!("Invalid outcome.")
            }
        }
    }

    println!("{:?}", teams.iter());

    String::from("")
}
