use std::collections::HashSet;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

pub fn run() {
    let input = include_str!("day04.input");

    println!("=== Day 04 ===");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
    println!();
}

#[derive(Debug)]
struct Sector {
    start: usize,
    end: usize,
    range: Vec<usize>,
}

impl FromStr for Sector {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split('-').collect();

        let start: usize = parts[0]
            .parse::<usize>()
            .unwrap_or_else(|_| panic!("start is not a number '{}'", parts[0]));

        let end: usize = parts[1]
            .parse::<usize>()
            .unwrap_or_else(|_| panic!("end is not a number '{}'", parts[1]));

        let mut sector = Sector {
            start,
            end,
            range: vec![],
        };
        for i in start..=end {
            sector.range.push(i);
        }

        Ok(sector)
    }
}

impl Sector {
    fn fully_contains(&self, other: &Sector) -> bool {
        // println!("testing if {} fully contains {}", self, other);
        self.start <= other.start && self.end >= other.end
    }

    fn overlaps(&self, other: &Sector) -> bool {
        let a: HashSet<usize> = self.range.clone().into_iter().collect();
        let b: HashSet<usize> = other.range.clone().into_iter().collect();

        let intersection = a.intersection(&b);
        intersection.count() > 0
    }
}

impl Display for Sector {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for i in 1..=9 {
            if self.range.contains(&i) {
                write!(f, "{}", i)?;
            } else {
                write!(f, ".")?;
            }
        }
        write!(f, " {}-{}", self.start, self.end)?;
        Ok(())
    }
}

fn part1(input: &str) -> usize {
    let pairs: Vec<Vec<Sector>> = input
        .lines()
        .map(|l| -> Vec<Sector> {
            l.split(',')
                .map(|x| x.parse::<Sector>().expect("not a valid sector"))
                .collect()
        })
        .collect();

    let mut result: usize = 0;

    for pair in pairs {
        let sector1 = &pair[0];
        let sector2 = &pair[1];
        if sector1.fully_contains(sector2) || sector2.fully_contains(sector1) {
            result += 1;
        }
    }

    result
}

fn part2(input: &str) -> usize {
    let pairs: Vec<Vec<Sector>> = input
        .lines()
        .map(|l| -> Vec<Sector> {
            l.split(',')
                .map(|x| x.parse::<Sector>().expect("not a valid sector"))
                .collect()
        })
        .collect();

    let mut result: usize = 0;

    for pair in pairs {
        let sector1 = &pair[0];
        let sector2 = &pair[1];
        if sector1.overlaps(sector2) {
            result += 1;
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    const TESTINPUT: &str = include_str!("day04.test");

    #[test]
    fn test_to_sector() {
        let sector = "2-4".parse::<Sector>().unwrap();
        assert_eq!(sector.range.len(), 3);
        assert_eq!(sector.range[0], 2);
        assert_eq!(sector.range[1], 3);
        assert_eq!(sector.range[2], 4);
    }

    #[test]
    fn test_fully_contains() {
        let sector1 = "2-8".parse::<Sector>().unwrap();
        let sector2 = "3-7".parse::<Sector>().unwrap();
        assert!(sector1.fully_contains(&sector2));
        assert!(!sector2.fully_contains(&sector1));
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(TESTINPUT), 2);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TESTINPUT), 4);
    }
}
