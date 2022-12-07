use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::alphanumeric0;
use nom::combinator::map_res;
use nom::multi::many1;
use nom::sequence::preceded;
use nom::sequence::tuple;
use nom::Parser;
use std::str::FromStr;

#[derive(Debug)]
enum LineItem<'a> {
    LS,
    CD(&'a str),
    DIR(&'a str),
    FILE(&'a str, u64),
}

fn number(input: &str) -> nom::IResult<&str, u64> {
    map_res(nom::character::complete::digit1, u64::from_str).parse(input)
}

fn parse_line<'a>(input: &'a str) -> nom::IResult<&'a str, LineItem<'a>> {
    let ls_statement = |input| tag("$ ls").map(|_| LineItem::LS).parse(input);
    let cd_statement = |input| {
        preceded(tag("$ cd "), nom::character::complete::not_line_ending)
            .map(|dir| LineItem::CD(dir))
            .parse(input)
    };

    let entry_dir = |input| {
        preceded(tag("dir "), nom::character::complete::not_line_ending)
            .map(LineItem::DIR)
            .parse(input)
    };

    let entry_file = |input| {
        tuple((number, tag(" "), nom::character::complete::not_line_ending))
            .map(|(size, _, name)| LineItem::FILE(name, size))
            .parse(input)
    };

    alt((ls_statement, cd_statement, entry_dir, entry_file)).parse(input)
}

type DirectoryT<'a> = Vec<Node<'a>>;

#[derive(Debug, Clone)]
enum Node<'a> {
    File {
        name: &'a str,
        size: u64,
    },
    Directory {
        name: &'a str,
        nodes: DirectoryT<'a>,
        size: u64,
    },
}

#[derive(Debug)]
struct Tree<'a> {
    nodes: DirectoryT<'a>,
    size: u64,
}

impl<'a> Tree<'a> {
    fn walk<X>(&'a self, f: impl Fn(&Node) -> Option<X>) -> Vec<X> {
        let mut stack = vec![self.nodes.iter()];
        let mut values = vec![];

        while stack.len() > 0 {
            let mut iter = stack
                .pop()
                .expect("We just had an entry, where did it go?!");
            let (value, s) = match iter.next() {
                Some(s) => (f(s), s),
                None => {
                    continue;
                }
            };
            stack.push(iter);

            if let Some(v) = value {
                values.push(v);
            }

            match s {
                Node::Directory { nodes, .. } => {
                    stack.push(nodes.iter());
                }
                _ => {}
            }
        }

        values
    }
}

// struct TreeIterator<'a, 'x> {
//     stack: Vec<std::slice::Iter<'x, Node<'a>>>,
// }
//
// impl<'a, 'x> TreeIterator<'a, 'x> {
//     fn new(tree: &'x Tree<'a>) -> Self {
// 	let it = tree.nodes.iter();
// 	Self { stack: vec![ it ]}
//     }
// }
//
// impl<'a, 'x> Iterator for TreeIterator<'a, 'x> {
//     type Item = &'x Node<'a>;
//
//     fn next(&self) -> Option<&'x Node<'a>> {
// 	let x = match self.stack.get_mut(0) {
// 	    None => return None,
// 	    Some(x) => x,
// 	};
//
// 	match x.next() {
// 	    Some(Node::File { .. }) @ x => return x,
// 	    Some(Node::Directory { .. }) @ x => return x,
//
// 	}
//     }
// }
//
// impl<'a, 'b> Tree<'a> {
//     fn iter(&'b self) -> TreeIterator {
// 	TreeIterator::new(self)
//     }
// }

fn run<'a, I: Iterator<Item = LineItem<'a>>>(
    level: usize,
    it: &mut I,
    tree: &mut DirectoryT<'a>,
) -> (u64, bool) {
    let mut total_size = 0;
    while let Some(item) = it.next() {
        match item {
            LineItem::FILE(name, s) => {
                total_size += s;
                tree.push(Node::File { name, size: s });
            }
            LineItem::DIR(subdir) => {
                let nodes = DirectoryT::new();
                tree.push(Node::Directory {
                    name: subdir,
                    nodes,
                    size: 0,
                });
            }
            LineItem::CD(subdir) if subdir == ".." => return (total_size, false),
            LineItem::CD(subdir) if subdir == "/" && level != 0 => return (total_size, true),
            LineItem::CD(subdir) if subdir == "/" && level == 0 => {}
            LineItem::CD(subdir) => {
                // find the node in the current tree nodes
                let x = tree.iter_mut().find(|n| match n {
                    Node::Directory { name, .. } if *name == subdir => true,
                    _ => false,
                });
                match x {
                    Some(Node::Directory {
                        ref mut nodes,
                        ref mut size,
                        ..
                    }) => {
                        let (sub_size, return_to_root) = run(level + 1, it, nodes);
                        total_size += sub_size;
                        *size = sub_size;
                        if return_to_root && level > 0 {
                            return (total_size, true);
                        }
                    }
                    None => panic!("Tried to enter directory that isn't known: {}", subdir),
                    _ => panic!("the fuck?"),
                }
            }
            _ => {}
        }
    }

    (total_size, false)
}

fn parse_to_tree<'a>(input: &'a str) -> Tree<'a> {
    let mut tree = Tree {
        nodes: DirectoryT::new(),
        size: 0,
    };

    let mut it = input.lines().map(|x| {
        let (_, x) = parse_line(x).unwrap();
        x
    });

    let (size, return_to_root) = run(0, &mut it, &mut tree.nodes);
    if return_to_root {
        panic!("return to root shouldn't bubble up");
    }
    tree.size = size;

    tree
}

fn part1<'a>(input: &'a str) -> u64 {
    let tree = parse_to_tree(input);

    let directories: Vec<u64> = tree.walk(|node| match node {
        Node::Directory { name, size, .. } if *size <= 100000 => Some(*size),
        _ => None,
    });

    let sum = directories
        .into_iter()
        .sum();

    sum
}

fn part2<'a>(input: &'a str) -> u64 {
    let disk_size = 70000000;
    let space_required = 30000000;

    let tree = parse_to_tree(input);

    let free = disk_size - tree.size;

    println!("free: {}", free);

    let have_to_free_at_least = space_required - free;
    println!("to free: {}", have_to_free_at_least);

    let directories: Vec<u64> = tree.walk(|node| match node {
        Node::Directory { name, size, .. } if *size >= have_to_free_at_least => Some(*size),
        _ => None,
    });

    let sum = directories.into_iter().min().unwrap();

    sum
}

fn main() {
    println!("part1: {}", part1(include_str!("input.txt")));
    println!("part2: {}", part2(include_str!("input.txt")));
}

#[cfg(test)]
mod tests {
    use super::*;
    const input: &'static str = r#"$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k"#;

    #[test]
    fn test_example() {
        assert_eq!(part1(input), 95437);
    }

    #[test]
    fn test_example_tree() {
        let t = parse_to_tree(input);
        assert_eq!(t.size, 48381165);
        assert_eq!(t.nodes.len(), 4);
        assert!(matches!(t.nodes[0], Node::Directory { name: "a", .. }));
        assert!(matches!(t.nodes[1], Node::File { name: "b.txt", .. }));
        assert!(matches!(t.nodes[2], Node::File { name: "c.dat", .. }));
        assert!(matches!(t.nodes[3], Node::Directory { name: "d", .. }));

        let a = match &t.nodes[0] {
            Node::Directory { nodes, .. } => nodes,
            _ => panic!("foo"),
        };

        assert_eq!(a.len(), 4);
        assert!(matches!(
            a[0],
            Node::Directory {
                name: "e",
                size: 584,
                ..
            }
        ));
        assert!(matches!(a[1], Node::File { name: "f", .. }));
        assert!(matches!(a[2], Node::File { name: "g", .. }));
        assert!(matches!(a[3], Node::File { name: "h.lst", .. }));
    }
}
