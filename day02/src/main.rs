use smallvec::SmallVec;

fn main() {
    let input  = include_str!("input.txt");
    println!("part1: {}", part1(input));
}

#[derive(Debug, Clone, Copy)]
enum Outcome {
    Lost,
    Draw,
    Win,
}

impl Outcome {
    fn score(&self) -> u64 {
        match self {
            Outcome::Lost => 0,
            Outcome::Draw => 3,
            Outcome::Win => 6,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Choice {
    Rock,
    Paper,
    Scissors,
}

impl Choice {
    fn shape_score(&self) -> u64 {
        match self {
            Choice::Rock => 1,
            Choice::Paper => 2,
            Choice::Scissors => 3,
        }
    }

    fn play(&self, other: &Choice) -> Outcome {
        match (self, other) {
            (Choice::Paper, Choice::Rock) => Outcome::Win,
            (Choice::Rock, Choice::Scissors) => Outcome::Win,
            (Choice::Scissors, Choice::Paper) => Outcome::Win,
            (x, y) if x == y => Outcome::Draw,
            _ => Outcome::Lost,
        }
    }
}

fn run_round(own: Choice, other: Choice) -> u64 {
    let outcome = own.play(&other);

    outcome.score() + own.shape_score()
}

impl From<&str> for Choice {
    fn from(other: &str) -> Choice {
        match other {
            "A" | "X" => Choice::Rock,
            "B" | "Y" => Choice::Paper,
            "C" | "Z" => Choice::Scissors,
            x => panic!("Unknown input: {}", x),
        }
    }
}

fn decode_row(input: &str) -> (Choice, Choice) {
    let data: SmallVec<[Choice; 2]> = input.split(' ').map(Choice::from).take(2).collect();

    // FIXME: unchecked bounds access
    assert_eq!(data.len(), 2);
    (data[0], data[1])
}

fn part1(input: &str) -> u64 {
    input
        .lines()
        .map(decode_row)
        .map(|(other, own)| run_round(own, other))
        .sum()
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_example() {
        let input = r#"A Y
B X
C Z"#;
        assert_eq!(super::part1(input), 15);
    }
}
