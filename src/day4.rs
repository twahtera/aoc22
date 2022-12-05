fn main() {
    const INPUT: &str = include_str!("../inputs/day4.txt");
    let input: Vec<(Range, Range)> = INPUT.lines().map(parse_line).collect();
    println!("Solution 1: {:?}", solve1(&input));
    println!("Solution 2: {:?}", solve2(&input));
}

fn solve1(input: &Vec<(Range, Range)>) -> i64 {
    input.iter().filter(|(a, b)| either_includes(a, b)).count() as i64
}

fn solve2(input: &Vec<(Range, Range)>) -> i64 {
    input.iter().filter(|(a, b)| overlaps(a, b)).count() as i64
}

type Range = (i64, i64);

fn parse_line(s: &str) -> (Range, Range) {
    let s2: Vec<&str> = s.split(',').collect();

    assert!(s2.len() == 2);

    (parse_range(s2[0]), parse_range(s2[1]))
}

fn parse_range(s: &str) -> Range {
    let s2: Vec<&str> = s.split('-').collect();
    assert!(s2.len() == 2);

    (s2[0].parse().unwrap(), s2[1].parse().unwrap())
}

fn either_includes(a: &Range, b: &Range) -> bool {
    includes(a, b) || includes(b, a)
}

fn includes((a, b): &Range, (x, y): &Range) -> bool {
    a <= x && b >= y
}

fn overlaps((a, b): &Range, (x, y): &Range) -> bool {
    (a <= x && x <= b) || (a <= y && y <= b) || (x < a && b < y)
}

#[test]
fn test_solve1() {
    const TEST_INPUT: &str = include_str!("../inputs/day4_test.txt");
    let input: Vec<(Range, Range)> = TEST_INPUT.lines().map(parse_line).collect();

    assert_eq!(solve1(&input), 2);
}

#[test]
fn test_solve2() {
    const TEST_INPUT: &str = include_str!("../inputs/day4_test.txt");
    let input: Vec<(Range, Range)> = TEST_INPUT.lines().map(parse_line).collect();

    assert_eq!(solve2(&input), 4);
}

#[test]
fn test_parse_line() {
    assert_eq!(parse_line("2-4,6-8"), ((2, 4), (6, 8)));
}
