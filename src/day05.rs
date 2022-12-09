use std::fmt::{Display, Formatter};
use std::str::FromStr;

pub fn run() {
    let input = include_str!("day05.input");

    println!("=== Day 05 ===");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
    println!();
}

struct Ship {
    stacks: Vec<Vec<char>>,
}

impl Ship {
    fn top_str(&self) -> String {
        let top: Vec<char> = self
            .stacks
            .iter()
            .map(|s| *s.last().unwrap_or(&' '))
            .collect();
        // println!("top: {:?}", top);

        top.into_iter().collect::<String>()
    }

    fn execute_move(&mut self, m: Move) {
        // println!("Moving: {}", m);
        let mut temp_stack: Vec<char> = vec![];
        for _i in 0..m.count {
            temp_stack.push(self.stacks[m.from - 1].pop().unwrap())
        }
        for c in temp_stack {
            self.stacks[m.to - 1].push(c);
        }
    }

    fn execute_move_9001(&mut self, m: Move) {
        // println!("Moving: {}", m);
        let mut temp_stack: Vec<char> = vec![];
        for _i in 0..m.count {
            temp_stack.insert(0, self.stacks[m.from - 1].pop().unwrap())
        }
        for c in temp_stack {
            self.stacks[m.to - 1].push(c);
        }
    }
}

impl FromStr for Ship {
    type Err = String;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut ship: Ship = Ship { stacks: vec![] };

        let stack_height = input.lines().position(|l| l.starts_with(" 1")).unwrap();
        // println!("stack height: {}", stack_height);

        let stack_count: usize = input
            .lines()
            .nth(stack_height)
            .unwrap()
            .split_ascii_whitespace()
            .count();

        // Create stacks on ship
        for _i in 0..stack_count {
            ship.stacks.push(vec![]);
        }

        for line in input.lines() {
            if line.starts_with(" 1") {
                break;
            }

            let chars: Vec<char> = line.chars().collect();

            // Iterate stacks
            for stack_index in 0..ship.stacks.len() {
                // Get stack crate index
                let ci = 1 + stack_index * 4;

                // No more chars? break
                if chars.len() < ci {
                    break;
                }

                // No crate? Skip
                if chars[ci] == ' ' {
                    // Skip empty
                    continue;
                }

                ship.stacks[stack_index].insert(0, chars[ci]);
            }
        }

        Ok(ship)
    }
}

impl Display for Ship {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for stack in &self.stacks {
            for c in stack {
                write!(f, "[{}] ", c)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

struct Move {
    from: usize,
    to: usize,
    count: usize,
}

impl Display for Move {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "|{}| -- [{}] -> |{}|", self.from, self.count, self.to)
    }
}

impl FromStr for Move {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split_ascii_whitespace().collect();
        if parts.len() != 6 {
            return Err("not enough data in line".to_string());
        }

        let count: usize = parts[1].parse().unwrap();
        let from: usize = parts[3].parse().unwrap();
        let to: usize = parts[5].parse().unwrap();

        Ok(Move { count, from, to })
    }
}

fn part1(input: &str) -> String {
    let mut ship = Ship::from_str(input).unwrap();

    // println!("Ship:\n{}", ship);

    let moves: Vec<Move> = input
        .lines()
        .filter(|l| l.starts_with("move"))
        .flat_map(Move::from_str)
        .collect();
    // println!("Moves:");
    for m in moves {
        // println!("  {}", m);
        ship.execute_move(m);
    }

    ship.top_str()
}

fn part2(input: &str) -> String {
    let mut ship = Ship::from_str(input).unwrap();

    let moves: Vec<Move> = input
        .lines()
        .filter(|l| l.starts_with("move"))
        .flat_map(Move::from_str)
        .collect();

    for m in moves {
        ship.execute_move_9001(m);
    }

    ship.top_str()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TESTINPUT: &str = include_str!("day05.test");

    #[test]
    fn test_top() {
        let ship = Ship::from_str(TESTINPUT).unwrap();
        assert_eq!(ship.top_str(), "NDP");
    }

    #[test]
    fn test_move() {
        let mut ship = Ship::from_str(TESTINPUT).unwrap();
        assert_eq!(ship.top_str(), "NDP");
        println!("Before:\n{}", ship);
        ship.execute_move(Move {
            count: 1,
            from: 2,
            to: 1,
        });
        println!("After:\n{}", ship);
        assert_eq!(ship.top_str(), "DCP");

        ship.execute_move(Move {
            count: 3,
            from: 1,
            to: 3,
        });
        println!("After:\n{}", ship);
        assert_eq!(ship.top_str(), " CZ");
    }

    #[test]
    fn test_move_9001() {
        let mut ship = Ship::from_str(TESTINPUT).unwrap();
        assert_eq!(ship.top_str(), "NDP");
        println!("Before:\n{}", ship);
        ship.execute_move_9001(Move {
            count: 1,
            from: 2,
            to: 1,
        });
        println!("After:\n{}", ship);
        assert_eq!(ship.top_str(), "DCP");

        ship.execute_move_9001(Move {
            count: 3,
            from: 1,
            to: 3,
        });
        println!("After:\n{}", ship);
        assert_eq!(ship.top_str(), " CD");
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(TESTINPUT), "CMZ");
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TESTINPUT), "MCD");
    }
}
