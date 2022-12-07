use itertools::Itertools;


fn find_marker(input: &str) -> usize {
    for (n, (a,b,c,d)) in input.chars().tuple_windows().enumerate() {
	println!("{} {} {} {}", a, b, c, d);
	if a != b && b != c && c != d
	    && a != b && a != c && a != d
	    && b != d {
		return n + 4;
	    }
    }

    input.len()
}

fn has_duplicates(input: &str) -> bool {
    let chars = input.as_bytes();
    for i in 0..input.len() {
	for x in 0..input.len() {
	    if x == i {
		continue
	    }

	    if chars[x] == chars[i] {
		return true;
	    }
	}
    }
    return false;
}

fn find_message(input: &str) -> usize {
    let search_length = 14;
    let start = 0;
    for i in start..input.len() {
	if input.len() - i < search_length {
	    return 0;
	}
	let candidate = &input[i..(i+search_length).clamp(0, input.len())];
	if !has_duplicates(candidate) {
	    return i + search_length;
	}
    }

    0
}

fn part1(input: &str) -> usize {
    find_marker(input)
}

fn part2(input: &str) -> usize {
    find_message(input)
}

fn main() {
    println!("part1: {}", part1(include_str!("input.txt")));
    println!("part2: {}", part2(include_str!("input.txt")));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_marker() {
	assert_eq!(find_marker("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 7);
        assert_eq!(find_marker("bvwbjplbgvbhsrlpgdmjqwftvncz"), 5);
        assert_eq!(find_marker("nppdvjthqldpwncqszvftbrmjlhg"), 6);
	assert_eq!(find_marker("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 10);
	assert_eq!(find_marker("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 11);
    }

    #[test]
    fn test_has_duplicates() {
	assert!(has_duplicates("abcdefgha"));
	assert!(!has_duplicates("abcdefgh"));
	assert!(!has_duplicates("abcedfghijklmnopqrstuvw"));
    }
    
    #[test]
    fn test_find_message() {
	assert_eq!(find_message("1234567890abcdefghijklmn"), 14);
	assert_eq!(find_message("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 19);
        assert_eq!(find_message("bvwbjplbgvbhsrlpgdmjqwftvncz"), 23);
        assert_eq!(find_message("nppdvjthqldpwncqszvftbrmjlhg"), 23);
	assert_eq!(find_message("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 29);
	assert_eq!(find_message("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 26);
    }

    #[test]
    fn test_find_start_of_message() {
    }
}
