use smallvec::SmallVec;

fn main() {
    let input = include_str!("input.txt");
    println!("part1: {}", part1(input));
    println!("part1: {}", part2(input));
}

#[derive(Debug, Clone, Copy, PartialEq)]
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

    fn pick_for_outcome(&self, outcome: &Outcome) -> Choice {
        match (self, outcome) {
            (x, Outcome::Draw) => *x,
            (Choice::Paper, Outcome::Win) => Choice::Scissors,
            (Choice::Rock, Outcome::Win) => Choice::Paper,
            (Choice::Scissors, Outcome::Win) => Choice::Rock,
            (Choice::Paper, Outcome::Lost) => Choice::Rock,
            (Choice::Rock, Outcome::Lost) => Choice::Scissors,
            (Choice::Scissors, Outcome::Lost) => Choice::Paper,
        }
    }
}

fn run_round(own: Choice, other: Choice) -> u64 {
    let outcome = own.play(&other);

    outcome.score() + own.shape_score()
}

impl From<&str> for Outcome {
    fn from(other: &str) -> Outcome {
        match other {
            "X" => Outcome::Lost,
            "Y" => Outcome::Draw,
            "Z" => Outcome::Win,
            x => panic!("Unknown input: {}", x),
        }
    }
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

fn decode_part2_row(input: &str) -> (Choice, Outcome) {
    let (a, b) = input.split_at(input.find(' ').expect("All input lines have a space"));
    let choice = Choice::from(a.trim());
    let outcome = Outcome::from(b.trim());
    (choice, outcome)
}

fn part1(input: &str) -> u64 {
    input
        .lines()
        .map(decode_row)
        .map(|(other, own)| run_round(own, other))
        .sum()
}

fn run_part2_round(opponent_choice: Choice, outcome: Outcome) -> u64 {
    let choice = opponent_choice.pick_for_outcome(&outcome);
    assert_eq!(choice.play(&opponent_choice), outcome);

    outcome.score() + choice.shape_score()
}

fn part2(input: &str) -> u64 {
    input
        .lines()
        .map(decode_part2_row)
        .map(|(other, outcome)| run_part2_round(other, outcome))
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
	assert_eq!(super::part2(input), 12);
    }
}
