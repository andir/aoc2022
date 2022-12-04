use itertools::Itertools;

fn main() {
    let input = include_str!("input.txt");
    println!("part1: {}", part1(input));
    println!("part2: {}", part2(input));
}

#[derive(Debug, Clone)]
struct Rucksack<'a> {
    comp1: &'a str,
    comp2: &'a str,
}

#[inline]
fn item_score(c: char) -> u64 {
    let value = u64::from(c);

    let a: u64 = u64::from('a');
    let z: u64 = u64::from('z');

    let A: u64 = u64::from('A');
    let Z: u64 = u64::from('Z');

    if value >= a && value <= z {
        (value - a) + 1
    } else if value >= A && value <= Z {
        (value - A) + 27
    } else {
        panic!("value out of range");
    }
}

impl<'a> Rucksack<'a> {
    fn parse(input: &'a str) -> Rucksack<'a> {
        let len = input.len(); // les just hope this is never zero
        let comp1 = &input[0..(len / 2)];
        let comp2 = &input[(len / 2)..];

        Self { comp1, comp2 }
    }

    fn score(&self) -> u64 {
        let set = self
            .comp1
            .chars()
            .collect::<std::collections::HashSet<char>>();

        set.into_iter()
            .filter(|c| self.comp2.contains(*c))
            .map(|c| item_score(c))
            .sum()
    }

    fn unique_items(&self) -> std::collections::HashSet<char> {
        self.comp1.chars().chain(self.comp2.chars()).collect()
    }
}

fn part1<'a>(input: &'a str) -> u64 {
    input.lines().map(Rucksack::parse).map(|r| r.score()).sum()
}

fn part2<'a>(input: &'a str) -> u64 {
    let it = input.lines().map(Rucksack::parse);

    let mut sum: u64 = 0;
    for rs in &it.chunks(3) {
        let common = rs
            .map(|rucksack| rucksack.unique_items())
            .reduce(|acc, r| acc.intersection(&r).cloned().collect());

	let common = common.expect("There must be something");

	assert_eq!(common.len(), 1);

	let s : u64 = common.iter().map(|i| item_score(*i)).sum();
	sum += s;
    }

    sum
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_value_to_score() {
        assert_eq!(super::item_score('a'), 1);
        assert_eq!(super::item_score('A'), 27);
        assert_eq!(super::item_score('Z'), 52);
        assert_eq!(super::item_score('z'), 26);
    }

    #[test]
    fn test_example() {
        let input = r#"vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw"#;
        assert_eq!(super::part1(input), 157);
        assert_eq!(super::part2(input), 70);
    }
}
