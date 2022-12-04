fn main() {
    const INPUT: &str = include_str!("../inputs/day2.txt");
    let input = parse_input(INPUT);

    println!("Day 2 solution 1: {:?}", solve1(&input));
    println!("Day 2 solution 2: {:?}", solve2(&input));
}

fn solve1(input: &[(char, char)]) -> i64 {
    input.iter().map(score1).sum()
}

fn solve2(input: &[(char, char)]) -> i64 {
    input.iter().map(score2).sum()
}

fn score1((a, b): &(char, char)) -> i64 {
    let shape_score = match b {
        'X' => 1,
        'Y' => 2,
        'Z' => 3,
        _ => 0,
    };

    let win_score = match (a, b) {
        ('A', 'Y') | ('B', 'Z') | ('C', 'X') => 6, // win
        ('A', 'X') | ('B', 'Y') | ('C', 'Z') => 3, // draw
        ('A', 'Z') | ('B', 'X') | ('C', 'Y') => 0, // lose
        (_, _) => 0,
    };

    shape_score + win_score
}

fn score2((a, b): &(char, char)) -> i64 {
    let win_score = match b {
        'X' => 0,
        'Y' => 3,
        'Z' => 6,
        _ => 0,
    };

    let shape_score = match (a, b) {
        ('A', 'Y') | ('B', 'X') | ('C', 'Z') => 1,
        ('A', 'Z') | ('B', 'Y') | ('C', 'X') => 2,
        ('A', 'X') | ('B', 'Z') | ('C', 'Y') => 3,
        (_, _) => 0,
    };

    shape_score + win_score
}

fn parse_line(line_str: &str) -> Option<(char, char)> {
    let split: Vec<&str> = line_str.split(' ').collect();

    Some((split[0].chars().next()?, split[1].chars().next()?))
}

fn parse_input(input_str: &str) -> Vec<(char, char)> {
    input_str.lines().filter_map(parse_line).collect()
}

#[test]
fn test_parser() {
    assert_eq!(parse_line("A X"), Some(('A', 'X')));

    const TEST_INPUT: &str = include_str!("../inputs/day2_test.txt");
    let expected = vec![('A', 'Y'), ('B', 'X'), ('C', 'Z')];

    assert_eq!(parse_input(TEST_INPUT), expected);
}

#[test]
fn test_solution() {
    const TEST_INPUT: &str = include_str!("../inputs/day2_test.txt");

    let input = parse_input(TEST_INPUT);
    assert_eq!(solve1(&input), 15);
}

#[test]
fn test_solution2() {
    const TEST_INPUT: &str = include_str!("../inputs/day2_test.txt");

    let input = parse_input(TEST_INPUT);
    assert_eq!(solve2(&input), 12);
}
