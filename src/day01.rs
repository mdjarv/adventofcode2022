pub fn run() {
    let input = include_str!("day01.input");

    println!("=== Day 01 ===");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
    println!();
}

fn get_sorted_calories(input: &str) -> Vec<i64> {
    // Split into groups by double-newline
    let mut calorie_sums: Vec<i64> = input.split("\n\n")
        .map(|x| {
            x.lines().flat_map(str::parse::<i64>).sum()
        }).collect();

    // Sort from biggest to smallest number
    calorie_sums.sort_by(|a, b| b.cmp(a));
    return calorie_sums;
}

fn part1(input: &str) -> i64 {
    let calorie_sums = get_sorted_calories(input);
    return calorie_sums[0];
}

fn part2(input: &str) -> i64 {
    let calorie_sums = get_sorted_calories(input);
    return calorie_sums.iter().take(3).sum();
}

#[cfg(test)]
mod tests {
    use super::*;

    const TESTINPUT: &str = include_str!("day01.test");

    #[test]
    fn test_part1() {
        assert_eq!(part1(TESTINPUT), 24000);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TESTINPUT), 45000);
    }
}
