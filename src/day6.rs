fn main() {
    const INPUT: &str = include_str!("../inputs/day6.txt");

    println!("Solution 1: {:}", solve1(INPUT));
    println!("Solution 2: {:}", solve2(INPUT));
}

fn solve1(s: &str) -> usize {
    let mut ws = s.as_bytes().windows(4);
    ws.position(|w| all_unique(Vec::from(w))).unwrap() + 4
}

fn solve2(s: &str) -> usize {
    let mut ws = s.as_bytes().windows(14);
    ws.position(|w| all_unique(Vec::from(w))).unwrap() + 14
}

fn all_unique(s: Vec<u8>) -> bool {
    for (i, c) in s.iter().enumerate() {
        for c2 in s[i + 1..].iter() {
            if c == c2 {
                return false;
            }
        }
    }
    true
}

#[test]
fn test_all_unique() {
    assert!(all_unique(Vec::from("abcd".as_bytes())));
    assert!(!all_unique(Vec::from("abca".as_bytes())));
}

#[test]
fn test_solve1() {
    assert_eq!(solve1("bvwbjplbgvbhsrlpgdmjqwftvncz"), 5);
    assert_eq!(solve1("nppdvjthqldpwncqszvftbrmjlhg"), 6);
    assert_eq!(solve1("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 10);
    assert_eq!(solve1("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 11);
}
