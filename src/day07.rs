pub fn run() {
    let input = include_str!("day07.input");

    println!("=== Day 07 ===");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
    println!();
}

fn part1(_input: &str) -> usize {
    todo!()
}

fn part2(_input: &str) -> usize {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TESTINPUT: &str = include_str!("day07.test");

    #[test]
    fn test_part1() {
        assert_eq!(part1(TESTINPUT), 95437);
    }
}
