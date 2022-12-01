use aoc_utils::MaxResult;

const INPUT_PART1: &'static str = include_str!("part1.txt");
fn parse_elve(input: &str) -> Result<u64, <u64 as std::str::FromStr>::Err> {
    let values = input
        .trim()
        // split each elves lines
        .split_ascii_whitespace()
        // parse each of the numbers, this returns a Result<u64, _>, we must unwrap that
        .map(|s| s.parse::<u64>())
        .collect::<Result<Vec<_>, _>>()?;
    // now turn the collection into an owning iterator again and return the sum of the elves calories
    Ok(values.into_iter().sum::<u64>())
}

fn part1(input: &str) -> u64 {
    // split elves by double newlines
    let elves = input.split_terminator("\n\n");

    elves.map(parse_elve).max_result().unwrap().unwrap()
}

fn part2(input: &str) -> u64 {
    // split elves by double newlines
    let elves = input.split_terminator("\n\n");

    let mut values = elves
        .map(parse_elve)
        .collect::<Result<Vec<_>, _>>()
        .unwrap();

    values.sort();
    values.iter().rev().take(3).sum()
}

fn main() {
    println!("part1: {}", part1(INPUT_PART1));
    println!("part2: {}", part2(INPUT_PART1));
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_example() {
        let input = r#"1000
2000
3000

4000

5000
6000

7000
8000
9000

10000"#;
        let n = super::part1(input);
        assert_eq!(n, 24000);
        let n = super::part2(input);
        assert_eq!(n, 45000);
    }
}
