use std::collections::HashMap;

const HEADER: &'static str = "Team                           | MP |  W |  D |  L |  P";

#[derive(Clone, Debug, Copy)]
pub struct Team<'a> {
    name: &'a str,
    wins: u32,
    losses: u32,
    draws: u32,
}

impl<'a> Team<'a> {
    fn new() -> Self {
        Self {
            name: "",
            wins: 0,
            losses: 0,
            draws: 0,
        }
    }
}

pub fn format_entry(team: &Team) -> String {
    let points = team.wins * 3 + team.draws * 1;
    let matches_played = team.wins + team.losses + team.draws;

    format!(
        "{: <31}|  {} |  {} |  {} |  {} |  {}",
        team.name, matches_played, team.wins, team.draws, team.losses, points
    )
}

pub fn tally(match_results: &str) -> String {
    if match_results.len() == 0 {
        return HEADER.to_owned();
    }

    let mut map: HashMap<&str, Team> = HashMap::new();
    let mut output = HEADER.to_owned();
    let games: Vec<Vec<&str>> = match_results
        .split("\n")
        .map(|game| game.split(";").collect::<Vec<&str>>())
        .collect();

    for game in &games {
        let game_result = *game.last().unwrap();
        let mut t1 = *map.get(game[0]).unwrap_or(&Team::new());
        let mut t2 = *map.get(game[1]).unwrap_or(&Team::new());
        t1.name = game[0];
        t2.name = game[1];

        match game_result {
            "win" => {
                t1.wins += 1;
                t2.losses += 1;
            }
            "loss" => {
                t1.losses += 1;
                t2.wins += 1;
            }
            _ => {
                t1.draws += 1;
                t2.draws += 1;
            }
        }

        map.insert(game[0], t1);
        map.insert(game[1], t2);
    }

    let mut parsed_teams: Vec<Team> = map.values().cloned().collect();
    parsed_teams.sort_by(|team1, team2| {
        if team1.wins == team2.wins {
            team1.name.partial_cmp(team2.name).unwrap()
        } else {
            team2.wins.partial_cmp(&team1.wins).unwrap()
        }
    });

    output += "\n";
    for team in &parsed_teams {
        let mut entry = format_entry(&team);
        entry.push_str("\n");
        output += &entry[..];
    }
    output = output[..output.len() - 1].to_owned();

    output
}
