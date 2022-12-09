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
    fn execute_move(&mut self, m: Move) {
        let mut temp_stack: Vec<char> = vec![];
        for _i in 0..m.count {
            for stack_index in 0..self.stacks[m.from].len() {
                if self.stacks[m.from][stack_index] == ' ' {
                    // skip empty
                    continue;
                }

                temp_stack.push(self.stacks[m.from][stack_index]);
                self.stacks[m.from][stack_index] = ' ';
            }
        }
        println!("temp stack: {:?}", temp_stack);
        // m.from
    }
}

impl FromStr for Ship {
    type Err = String;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut ship: Ship = Ship { stacks: vec![] };

        let stack_height = input.lines().position(|l| l.starts_with(" 1")).unwrap();
        // println!("stack height: {}", stack_height);

        let stack_index: Vec<&str> = input
            .lines()
            .nth(stack_height)
            .unwrap()
            .split_ascii_whitespace()
            .collect();

        // println!("found stack index: {:?}", stack_index);

        for _i in 0..stack_index.len() {
            let mut stack: Vec<char> = vec![];
            for _j in 0..stack_height {
                stack.push(' ');
            }
            ship.stacks.push(stack);
        }

        for i in 0..ship.stacks[0].len() {
            let chars: Vec<char> = input.lines().nth(i).unwrap().chars().collect();
            for stack_index in 0..ship.stacks.len() {
                let ci = 1 + stack_index * 4;
                if chars.len() < ci {
                    break;
                }

                ship.stacks[stack_index][i] = chars[ci];
            }
        }

        Ok(ship)
    }
}

impl Display for Ship {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for height in 0..self.stacks[0].len() {
            for stack in &self.stacks {
                write!(f, "[{}] ", stack[height])?;
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

fn part1(input: &str) -> &str {
    let ship = Ship::from_str(input).unwrap();

    println!("Ship:\n{}", ship);

    let moves: Vec<Move> = input.lines().flat_map(Move::from_str).collect();
    println!("Moves:");
    for m in moves {
        println!("  {}", m);
    }
    ""
}

fn part2(input: &str) -> &str {
    unimplemented!("{}", input)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TESTINPUT: &str = include_str!("day05.test");

    #[test]
    fn test_move() {
        let mut ship = Ship {
            stacks: vec![
                vec![' ', 'N', 'Z'],
                vec!['D', 'C', 'M'],
                vec![' ', ' ', 'P'],
            ],
        };

        println!("Before:\n{}", ship);
        ship.execute_move(Move {
            count: 1,
            from: 2,
            to: 1,
        });
        println!("After:\n{}", ship);

        assert_eq!(ship.stacks[0], vec!['D', 'N', 'Z']);
        assert_eq!(ship.stacks[0], vec![' ', 'C', 'M']);
        assert_eq!(ship.stacks[0], vec![' ', ' ', 'P']);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(TESTINPUT), "CMZ");
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TESTINPUT), "");
    }
}
