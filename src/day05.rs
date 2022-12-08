pub fn run() {
    let input = include_str!("day05.input");

    println!("=== Day 05 ===");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
    println!();
}

fn part1(input: &str) -> usize {
    unimplemented!("{}", input)
}

fn part2(input: &str) -> usize {
    unimplemented!("{}", input)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TESTINPUT: &str = include_str!("day05.test");

    #[test]
    fn test_part1() {
        assert_eq!(part1(TESTINPUT), 0);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TESTINPUT), 0);
    }
}
