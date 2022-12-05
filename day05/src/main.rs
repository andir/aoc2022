use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::map_res;
use nom::sequence::tuple;
use nom::Parser;
use std::str::FromStr;

#[derive(Debug, PartialEq, Copy, Clone)]
struct Crate(char);

#[derive(Debug, PartialEq, Clone)]
struct Stack {
    items: Vec<Crate>,
}

#[derive(Debug, PartialEq)]
struct CargoBay {
    stacks: Vec<Stack>,
}

impl CargoBay {
    fn execute(&mut self, m: Move) {
        for _ in 0..(m.quantity) {
            match self.stacks[m.from - 1].items.pop() {
                Some(c) => {
                    println!("{:?} {:?}", m, c);
                    self.stacks[m.to - 1].items.push(c);
                }
                None => panic!("Running out of crates!"),
            }
        }
    }

    fn execute_bulk(&mut self, m: Move) {
        let src = &mut self.stacks[m.from - 1].items;
        let items = (0..(m.quantity))
            .filter_map(|_| src.pop())
            .collect::<Vec<_>>();
        println!("{:?} {:?}", m, items);
        for item in items.iter().rev() {
            self.stacks[m.to - 1].items.push(*item);
        }
    }

    fn top_of_stacks(&self) -> Vec<Crate> {
        self.stacks
            .iter()
            .filter_map(|stack| stack.items.last().cloned())
            .collect()
    }

    fn print(&self) {
        for (i, stack) in self.stacks.iter().enumerate() {
            let crates = stack
                .items
                .iter()
                .fold(String::new(), |s, c| format!("{} [{}]", s, c.0));
            println!("{} {}", i + 1, crates);
        }
    }
}

#[derive(Debug, PartialEq)]
struct Move {
    from: usize,
    to: usize,
    quantity: usize,
}

impl Move {
    fn parse(input: &str) -> nom::IResult<&str, Move> {
        let digit = |input| map_res(nom::character::complete::digit1, usize::from_str).parse(input);
        tuple((
            tag("move "),
            digit,
            tag(" from "),
            digit,
            tag(" to "),
            digit,
        ))
        .map(|(_, quantity, _, from, _, to)| Move { quantity, from, to })
        .parse(input)
    }
}

#[derive(Debug, PartialEq)]
struct Moves {
    moves: Vec<Move>,
}

#[derive(Debug, PartialEq)]
enum LineItem {
    Empty,
    Crate(Crate),
}

impl LineItem {
    fn parse(input: &str) -> nom::IResult<&str, LineItem> {
        let item =
            tuple((tag("["), nom::character::complete::anychar, tag("]"))).map(|(_, c, _)| c);
        alt((
            tag("   ").map(|_| LineItem::Empty),
            item.map(|c| LineItem::Crate(Crate(c))),
        ))
        .parse(input)
    }
}

fn parse_line(input: &str) -> nom::IResult<&str, Vec<LineItem>> {
    nom::multi::separated_list1(tag(" "), LineItem::parse).parse(input)
}

fn parse_stack_number(input: &str) -> nom::IResult<&str, u64> {
    tuple((
        tag(" "),
        map_res(nom::character::complete::digit1, u64::from_str),
        tag(" "),
    ))
    .map(|(_, n, _)| n)
    .parse(input)
}

fn parse_newlines(input: &str) -> nom::IResult<&str, ()> {
    nom::multi::many1(tag("\n")).map(|_| ()).parse(input)
}

fn parse_input(input: &str) -> (Vec<Vec<LineItem>>, Vec<u64>, Moves) {
    // parse the state until an empty line, from there on parse moves

    // each line contains a crate or three whitespaces for an empty spot,
    // after the parsing we transpose the lines into crates

    let (rem, cargo) = nom::multi::many1(nom::sequence::terminated(
        parse_line,
        nom::character::complete::newline,
    ))
    .parse(input)
    .unwrap();

    let (rem, numbers): (_, Vec<u64>) = nom::multi::separated_list1(tag(" "), parse_stack_number)
        .parse(rem)
        .unwrap();

    let (rem, _) = parse_newlines(rem).unwrap();

    let (rem, moves) = nom::multi::many1(
        tuple((
            Move::parse,
            nom::combinator::opt(nom::character::complete::newline),
        ))
        .map(|(a, _)| a),
    )
    .parse(rem)
    .unwrap();

    (cargo, numbers, Moves { moves })
}

fn transpose_cargo(numbers: Vec<u64>, cargo: Vec<Vec<LineItem>>) -> CargoBay {
    // the length of the numbers vec tells us how many stacks there will be
    let num_stacks = numbers.len();

    let mut stacks = vec![Stack { items: vec![] }; num_stacks];

    // iterate over the lines in reverse as that allows us to keep the
    // insertion order in the stacks
    for line in cargo.iter().rev() {
        for (stack_number, item) in line.iter().enumerate() {
            assert!(stack_number < num_stacks);
            match item {
                LineItem::Empty => {}
                LineItem::Crate(c) => stacks[stack_number].items.push(*c),
            };
        }
    }

    CargoBay { stacks }
}

fn main() {
    println!(
        "part1: {:?}",
        part1(include_str!("input.txt"))
            .into_iter()
            .fold(String::new(), |s, c| format!("{}{}", s, c.0))
    );

    println!(
        "part2: {:?}",
        part2(include_str!("input.txt"))
            .into_iter()
            .fold(String::new(), |s, c| format!("{}{}", s, c.0))
    );

}

fn part1(input: &str) -> Vec<Crate> {
    let (cargo, numbers, moves) = parse_input(input);
    let mut bay = transpose_cargo(numbers, cargo);
    for m in moves.moves {
        bay.execute(m);
        bay.print();
    }
    bay.top_of_stacks()
}

fn part2(input: &str) -> Vec<Crate> {
    let (cargo, numbers, moves) = parse_input(input);
    let mut bay = transpose_cargo(numbers, cargo);
    for m in moves.moves {
        bay.execute_bulk(m);
        bay.print();
    }
    bay.top_of_stacks()
}

#[cfg(test)]
mod tests {
    use super::*;
    const example: &'static str = r#"    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2"#;

    #[test]
    fn test_example() {
        assert_eq!(
            super::part1(example),
            vec![Crate('C'), Crate('M'), Crate('Z')]
        );

        assert_eq!(
            super::part2(example),
            vec![Crate('M'), Crate('C'), Crate('D')]
        );
    }

    #[test]
    fn test_parse_example() {
        let (cargo, numbers, moves) = parse_input(example);
        assert_eq!(
            moves.moves,
            vec![
                Move {
                    quantity: 1,
                    from: 2,
                    to: 1
                },
                Move {
                    quantity: 3,
                    from: 1,
                    to: 3
                },
                Move {
                    quantity: 2,
                    from: 2,
                    to: 1
                },
                Move {
                    quantity: 1,
                    from: 1,
                    to: 2
                },
            ]
        );
        assert_eq!(numbers, vec![1, 2, 3]);
        assert_eq!(
            cargo,
            vec![
                vec![
                    LineItem::Empty,
                    LineItem::Crate(Crate('D')),
                    LineItem::Empty,
                ],
                vec![
                    LineItem::Crate(Crate('N')),
                    LineItem::Crate(Crate('C')),
                    LineItem::Empty
                ],
                vec![
                    LineItem::Crate(Crate('Z')),
                    LineItem::Crate(Crate('M')),
                    LineItem::Crate(Crate('P'))
                ],
            ]
        );
    }

    #[test]
    fn test_transpose_cargo() {
        let cargo = vec![
            vec![
                LineItem::Empty,
                LineItem::Crate(Crate('D')),
                LineItem::Empty,
            ],
            vec![
                LineItem::Crate(Crate('N')),
                LineItem::Crate(Crate('C')),
                LineItem::Empty,
            ],
            vec![
                LineItem::Crate(Crate('Z')),
                LineItem::Crate(Crate('M')),
                LineItem::Crate(Crate('P')),
            ],
        ];
        let numbers = vec![1, 2, 3];

        let bay = transpose_cargo(numbers, cargo);
        assert_eq!(bay.stacks.len(), 3);
        assert_eq!(
            bay.stacks[0],
            Stack {
                items: vec![Crate('Z'), Crate('N')]
            }
        );
        assert_eq!(
            bay.stacks[1],
            Stack {
                items: vec![Crate('M'), Crate('C'), Crate('D')]
            }
        );
        assert_eq!(
            bay.stacks[2],
            Stack {
                items: vec![Crate('P')]
            }
        );
    }

    #[test]
    fn test_parse_line() {
        let line = "    [A]    ";
        let (_, line) = parse_line(line).unwrap();
        assert_eq!(line.len(), 3);
        assert_eq!(
            line,
            vec![
                LineItem::Empty,
                LineItem::Crate(Crate('A')),
                LineItem::Empty
            ]
        );

        let line = "    [A] [B]";
        let (_, line) = parse_line(line).unwrap();
        assert_eq!(line.len(), 3);
        assert_eq!(
            line,
            vec![
                LineItem::Empty,
                LineItem::Crate(Crate('A')),
                LineItem::Crate(Crate('B'))
            ]
        );
    }

    #[test]
    fn test_parse_input() {
        let input = "    [A]    \n[B] [C] [D]\n 1   2   3 \n\nmove 1 from 1 to 1";
        let (cargo, numbers, moves) = super::parse_input(input);
        assert_eq!(numbers, vec![1, 2, 3]);
        assert_eq!(
            cargo,
            vec![
                vec![
                    LineItem::Empty,
                    LineItem::Crate(Crate('A')),
                    LineItem::Empty
                ],
                vec![
                    LineItem::Crate(Crate('B')),
                    LineItem::Crate(Crate('C')),
                    LineItem::Crate(Crate('D'))
                ],
            ]
        );
    }

    #[test]
    fn test_parse_line_item() {
        let empty = "   ";
        let (r, res) = super::LineItem::parse(empty).unwrap();
        assert_eq!(r, "");
        assert_eq!(res, super::LineItem::Empty);

        let some = "[A]";
        let (r, res) = super::LineItem::parse(some).unwrap();
        assert_eq!(r, "");
        assert_eq!(res, super::LineItem::Crate(Crate('A')));
    }

    #[test]
    fn test_parse_move() {
        let m = "move 1 from 2 to 1";
        let (rem, res) = Move::parse(m).unwrap();
        assert_eq!(rem, "");
        assert_eq!(
            res,
            Move {
                from: 2,
                to: 1,
                quantity: 1
            }
        );
    }
}
