use std::collections::HashSet;

pub fn run() {
    let input = include_str!("day03.input");

    println!("=== Day 03 ===");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
    println!();
}

trait Prioritize {
    fn priority(&self) -> usize;
}

impl Prioritize for char {
    fn priority(&self) -> usize {
        match self.is_uppercase() {
            true => *self as usize - 38,
            false => *self as usize - 96,
        }
    }
}

fn part1(input: &str) -> usize {
    let result = input.lines().map(|l| -> usize {
        let mid = l.len() / 2;

        let first: HashSet<char> = l.chars().take(mid).collect();
        let second: HashSet<char> = l.chars().skip(mid).collect();

        let intersection: HashSet<char> = first.intersection(&second).cloned().collect();

        let c: char = intersection
            .into_iter()
            .next()
            .expect("should have at least one intersection");
        c.priority()
    });

    result.sum()
}

fn part2(_: &str) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    const TESTINPUT: &str = include_str!("day03.test");

    #[test]
    fn test_priority() {
        assert_eq!('a'.priority(), 1);
        assert_eq!('z'.priority(), 26);
        assert_eq!('A'.priority(), 27);
        assert_eq!('Z'.priority(), 52);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(TESTINPUT), 157);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TESTINPUT), 70);
        unimplemented!();
    }
}
