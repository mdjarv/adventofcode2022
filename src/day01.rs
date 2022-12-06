pub fn run() {
    println!("=== Day 01 ===");
    let input = include_str!("day01.input");
    println!("Part 1: {}", part1(input))
}

fn part1(input: &str) -> i64 {
    let result: Vec<i64> = input.split("\n\n")
        .map(|x| {
            return x.lines().flat_map(str::parse::<i64>).sum();
        }).collect();
    println!("result: {:?}", result);
    return 0;
}

#[cfg(test)]
mod tests {
    use super::*;

    const TESTINPUT: &str = include_str!("day01.test");

    #[test]
    fn test_part1() {
        assert_eq!(part1(TESTINPUT), 24000);
    }
}
