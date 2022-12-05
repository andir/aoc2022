use nom::character::complete::{char, digit1, newline};
use nom::combinator::map_res;
use nom::sequence::tuple;
use nom::Parser;
use std::str::FromStr;

#[derive(Debug)]
pub enum Error {
    ParsingRange,
    NotEnoughPartsInRange,
}

struct Range {
    range: std::ops::Range<u64>,
}

impl Range {
    fn parse(input: &str) -> nom::IResult<&str, Range> {
        let digit = |input| map_res(digit1, u64::from_str)(input);
        tuple((digit, char('-'), digit))
            .map(|(fst, _, lst)| Range { range: fst..lst })
            .parse(input)
    }

    #[inline]
    fn fully_contains(&self, other: &Self) -> bool {
        self.range.start <= other.range.start && self.range.end >= other.range.end
    }

    #[inline]
    fn overlaps(&self, other: &Self) -> bool {
        self.range.contains(&other.range.start)
            || self.range.contains(&other.range.end)
            || self.fully_contains(&other)
    }
}

struct Assignment {
    range1: Range,
    range2: Range,
}

impl Assignment {
    fn parse(input: &str) -> nom::IResult<&str, Self> {
        tuple((Range::parse, char(','), Range::parse))
            .map(|(range1, _, range2)| Assignment { range1, range2 })
            .parse(input)
    }

    fn fully_contains(&self) -> bool {
        self.range1.fully_contains(&self.range2) || self.range2.fully_contains(&self.range1)
    }

    fn overlaps(&self) -> bool {
	self.range1.overlaps(&self.range2) || self.range2.overlaps(&self.range1)
    }
}

fn parse_input(input: &str) -> nom::IResult<&str, Vec<Assignment>> {
    nom::multi::separated_list1(newline, Assignment::parse).parse(input)
}

fn part1(input: &str) -> usize {
    let (_, assignments) = parse_input(input).unwrap();

    assignments
        .into_iter()
        .filter(Assignment::fully_contains)
        .count()
}

fn part2(input: &str) -> usize {
    let (_, assignments) = parse_input(input).unwrap();

    assignments
        .into_iter()
        .filter(Assignment::overlaps)
        .count()
}


fn main() {
    println!("part1: {}", part1(include_str!("input.txt")));
    println!("part2: {}", part2(include_str!("input.txt")));
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_example() {
        let input = r#"2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8"#;
        assert_eq!(super::part1(input), 2);
        assert_eq!(super::part2(input), 4);
    }

    #[test]
    fn test_parse_range() {
        let (_, r) = super::Range::parse("1-5").unwrap();
        assert_eq!(r.range, 1..5);
    }

    #[test]
    fn test_parse_assignment() {
        let (_, r) = super::Assignment::parse("1-5,3-5").unwrap();
        assert_eq!(r.range1.range, 1..5);
        assert_eq!(r.range2.range, 3..5);
    }

    #[test]
    fn test_parse_input() {
        let (rem, l) = super::parse_input("1-2,3-5\n4-5,3-6").unwrap();
        assert_eq!(rem, "");
        assert_eq!(l.len(), 2);
        assert_eq!(l[0].range1.range.start, 1);
        assert_eq!(l[0].range1.range.end, 2);
        assert_eq!(l[0].range2.range.start, 3);
        assert_eq!(l[0].range2.range.end, 5);

        assert_eq!(l[1].range1.range.start, 4);
        assert_eq!(l[1].range1.range.end, 5);
        assert_eq!(l[1].range2.range.start, 3);
        assert_eq!(l[1].range2.range.end, 6);
    }

    #[test]
    fn test_fully_contains() {
        let a = super::Range { range: 1..5 };
        let b = super::Range { range: 2..3 };

        assert!(a.fully_contains(&b));
        assert!(!b.fully_contains(&a));

        let a = super::Assignment {
            range1: a,
            range2: b,
        };
        assert!(a.fully_contains());
    }
}
