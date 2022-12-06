use std::str::FromStr;

pub fn run() {
    let input = include_str!("day02.input");

    println!("=== Day 02 ===");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
    println!();
}

#[derive(Debug, PartialEq)]
enum Action {
    Rock,
    Paper,
    Scissors,
}

impl Action {
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
            "A" => Ok(Action::Rock),
            "B" => Ok(Action::Paper),
            "C" => Ok(Action::Scissors),
            _ => Err("invalid input".to_string())
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

impl FromStr for Play {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(" ");

        let opponent: Action = match parts.next().unwrap() {
            "A" => Action::Rock,
            "B" => Action::Paper,
            "C" => Action::Scissors,
            _ => return Err("invalid opponent input".to_string())
        };

        let player: Action = match parts.next().unwrap() {
            "X" => Action::Rock,
            "Y" => Action::Paper,
            "Z" => Action::Scissors,
            _ => return Err("invalid player input".to_string())
        };

        Ok(Play {
            opponent,
            player,
        })
    }
}

#[derive(Debug)]
enum PlayResult {
    Win,
    Loose,
    Draw,
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
            }
            Action::Paper => match self.opponent {
                Action::Rock => PlayResult::Win,
                _ => PlayResult::Loose,
            }
            Action::Scissors => match self.opponent {
                Action::Paper => PlayResult::Win,
                _ => PlayResult::Loose,
            }
        }
    }

    fn score(&self) -> usize {
        self.result().score() + self.player.score()
    }
}

fn part1(input: &str) -> usize {
    let plays: Vec<Play> = input.lines().map(|l| Play::from_str(l).expect("failed to parse")).collect();
    let results: Vec<usize> = plays.iter().map(|p| { p.score() }).collect();
    return results.iter().sum();
}

fn part2(_: &str) -> usize {
    return 0;
}

#[cfg(test)]
mod tests {
    use super::*;

    const TESTINPUT: &str = include_str!("day02.test");

    #[test]
    fn test_part1() {
        assert_eq!(part1(TESTINPUT), 15);
    }

    // #[test]
    // fn test_part2() {
    //     assert_eq!(part2(TESTINPUT), 45000);
    // }
}
