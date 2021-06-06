pub mod team;
pub mod team_league;

use team::Team;
use team_league::League;

pub fn tally(match_results: &str) -> String {
    let mut team_league = League::new();
    for l in match_results.split("\n").into_iter() {
        let game = l.split(";").collect::<Vec<&str>>();

        if game.len() == 3 {
            if let Some(outcome) = game.get(2) {
                match *outcome {
                    "win" => {
                        team_league.update_winning_team(*game.get(0).unwrap());
                        team_league.update_losing_team(*game.get(1).unwrap());
                    }
                    "loss" => {
                        team_league.update_winning_team(*game.get(1).unwrap());
                        team_league.update_losing_team(*game.get(0).unwrap())
                    }
                    "draw" => {
                        team_league.update_draw_teams(*game.get(0).unwrap(), *game.get(1).unwrap());
                    }
                    _ => {
                        panic!("Invalid input");
                    }
                }
            }
        } else {
            panic!("Invalid input");
        }
    }

    String::from(team_league)
}
