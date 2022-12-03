fn main() {
    let input = include_str!("input.txt");
    println!("part1: {}", part1(input));
}

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
}

fn part1<'a>(input: &'a str) -> u64 {
    input.lines().map(Rucksack::parse).map(|r| r.score()).sum()
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
    }
}
