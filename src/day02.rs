use std::error::Error;
use std::str::FromStr;

pub fn run() {
    let input = include_str!("day02.input");

    println!("=== Day 02 ===");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
    println!();
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum Action {
    Rock,
    Paper,
    Scissors,
}

impl Action {
    fn action_from_result(&self, desired_result: &PlayResult) -> Action {
        match desired_result {
            PlayResult::Loose => match self {
                Action::Rock => Action::Scissors,
                Action::Paper => Action::Rock,
                Action::Scissors => Action::Paper,
            },
            PlayResult::Win => match self {
                Action::Rock => Action::Paper,
                Action::Paper => Action::Scissors,
                Action::Scissors => Action::Rock,
            },
            PlayResult::Draw => *self,
        }
    }

    fn score(&self) -> usize {
        match self {
            Action::Rock => 1,
            Action::Paper => 2,
            Action::Scissors => 3,
        }
    }
}

impl FromStr for Action {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" | "X" => Ok(Action::Rock),
            "B" | "Y" => Ok(Action::Paper),
            "C" | "Z" => Ok(Action::Scissors),
            _ => Err("invalid input".to_string()),
        }
    }
}

impl ToString for Action {
    fn to_string(&self) -> String {
        match self {
            Action::Rock => "Rock".to_string(),
            Action::Paper => "Paper".to_string(),
            Action::Scissors => "Scissors".to_string(),
        }
    }
}

#[derive(Debug)]
struct Play {
    opponent: Action,
    player: Action,
}

#[derive(Debug)]
enum PlayResult {
    Win,
    Loose,
    Draw,
}

impl FromStr for PlayResult {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(PlayResult::Loose),
            "Y" => Ok(PlayResult::Draw),
            "Z" => Ok(PlayResult::Win),
            _ => Err("invalid input".to_string()),
        }
    }
}

impl PlayResult {
    fn score(&self) -> usize {
        match self {
            PlayResult::Win => 6,
            PlayResult::Draw => 3,
            PlayResult::Loose => 0,
        }
    }
}

impl Play {
    fn result(&self) -> PlayResult {
        if self.player == self.opponent {
            return PlayResult::Draw;
        }

        match self.player {
            Action::Rock => match self.opponent {
                Action::Scissors => PlayResult::Win,
                _ => PlayResult::Loose,
            },
            Action::Paper => match self.opponent {
                Action::Rock => PlayResult::Win,
                _ => PlayResult::Loose,
            },
            Action::Scissors => match self.opponent {
                Action::Paper => PlayResult::Win,
                _ => PlayResult::Loose,
            },
        }
    }

    fn score(&self) -> usize {
        self.result().score() + self.player.score()
    }
}

fn part1(input: &str) -> usize {
    let plays: Vec<Play> = input
        .lines()
        .map(|l| -> Result<Play, Box<dyn Error>> {
            let mut parts = l.split(' ');

            let opponent = Action::from_str(parts.next().unwrap())?;
            let player = Action::from_str(parts.next().unwrap())?;

            Ok(Play { opponent, player })
        })
        .into_iter()
        .flatten()
        .collect();

    let results: Vec<usize> = plays.into_iter().map(|p| p.score()).collect();

    results.iter().sum()
}

fn part2(input: &str) -> usize {
    let plays: Vec<Play> = input
        .lines()
        .map(|l| -> Result<Play, Box<dyn Error>> {
            let mut parts = l.split(' ');

            let opponent: Action = Action::from_str(parts.next().unwrap())?;
            let desired_result: PlayResult = PlayResult::from_str(parts.next().unwrap())?;
            let player: Action = opponent.action_from_result(&desired_result);

            Ok(Play { opponent, player })
        })
        .into_iter()
        .flatten()
        .collect();

    plays.into_iter().map(|p| p.score()).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TESTINPUT: &str = include_str!("day02.test");

    #[test]
    fn test_part1() {
        assert_eq!(part1(TESTINPUT), 15);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TESTINPUT), 12);
    }
}
