use nom::character::complete::digit1;
use nom::combinator::map;
use nom::{
    branch::alt, bytes::complete::tag, combinator::map_res, multi::separated_list0,
    sequence::delimited, IResult,
};
use std::cmp::Ordering;
use std::iter::zip;

#[derive(Eq, Debug, PartialEq, Clone)]
enum Signal {
    I(i64),
    L(Vec<Signal>),
}

impl Ord for Signal {
    fn cmp(&self, other: &Signal) -> Ordering {
        fn cmp_vecs(s: &Vec<Signal>, o: &Vec<Signal>) -> Ordering {
            for (a, b) in zip(s, o) {
                if a < b {
                    return Ordering::Less;
                } else if a > b {
                    return Ordering::Greater;
                }
            }
            (s.len() as i64).cmp(&(o.len() as i64))
        }

        match (self, other) {
            (Signal::I(s), Signal::I(o)) => s.cmp(o),
            (Signal::L(s), Signal::L(o)) => cmp_vecs(s, o),
            (Signal::I(s), o) => Signal::L(vec![Signal::I(*s)]).cmp(o),
            (s, Signal::I(o)) => s.cmp(&Signal::L(vec![Signal::I(*o)])),
        }
    }
}

impl PartialOrd for Signal {
    fn partial_cmp(&self, other: &Signal) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn parse_signal(input: &str) -> IResult<&str, Signal> {
    alt((parse_l, parse_i))(input)
}

fn parse_i(input: &str) -> IResult<&str, Signal> {
    map_res(digit1, i_from_str)(input)
}

fn i_from_str(input: &str) -> Result<Signal, <i64 as std::str::FromStr>::Err> {
    Ok(Signal::I(input.parse::<i64>()?))
}

fn parse_l(input: &str) -> IResult<&str, Signal> {
    let parse_inner = separated_list0(tag(","), parse_signal);
    map(delimited(tag("["), parse_inner, tag("]")), Signal::L)(input)
}

fn parse(input: &str) -> Vec<(Signal, Signal)> {
    let input_pairs: Vec<Vec<&str>> = input.split("\n\n").map(|s| s.lines().collect()).collect();
    input_pairs
        .iter()
        .map(|p| (parse_signal(p[0]).unwrap().1, parse_signal(p[1]).unwrap().1))
        .collect()
}

fn parse2(input: &str) -> Vec<Signal> {
    input
        .lines()
        .filter_map(|l| parse_signal(l).ok())
        .map(|r| r.1)
        .collect()
}

fn solve1(input: &Vec<(Signal, Signal)>) -> i64 {
    input
        .iter()
        .enumerate()
        .filter(|(_, (a, b))| a < b)
        .map(|(i, _)| i as i64 + 1)
        .sum()
}

fn solve2(input: &mut Vec<Signal>) -> i64 {
    let mark1 = parse_signal("[[2]]").unwrap().1;
    let mark2 = parse_signal("[[6]]").unwrap().1;
    input.push(mark1.clone());
    input.push(mark2.clone());
    input.sort();

    let i1 = input.iter().position(|s| *s == mark1).unwrap_or(0) as i64 + 1;
    let i2 = input.iter().position(|s| *s == mark2).unwrap_or(0) as i64 + 1;

    i1 * i2
}

fn main() {
    const INPUT: &str = include_str!("../inputs/day13.txt");
    let parsed = parse(INPUT);
    let solution1 = solve1(&parsed);
    let mut parsed2 = parse2(INPUT);
    let solution2 = solve2(&mut parsed2);

    println!("Solution 1: {:}", solution1);
    println!("Solution 2: {:}", solution2);
}

#[test]
fn test_parse_signal() {
    assert_eq!(parse_signal("[]"), Ok(("", Signal::L(vec![]))));
    assert_eq!(
        parse_signal("[1,2,3]"),
        Ok((
            "",
            Signal::L(vec![Signal::I(1), Signal::I(2), Signal::I(3)])
        ))
    );
    assert_eq!(
        parse_signal("[1,[2]]"),
        Ok((
            "",
            Signal::L(vec![Signal::I(1), Signal::L(vec![Signal::I(2)])])
        ))
    )
}

#[test]
fn test_signal_ord() {
    assert!(Signal::I(1) < Signal::I(2));

    assert_eq!(
        parse_signal("[1,1]").unwrap(),
        parse_signal("[1,1]").unwrap()
    );

    assert!(parse_signal("[]").unwrap() < parse_signal("[3]").unwrap());
    assert!(parse_signal("[9]").unwrap() > parse_signal("[8,1]").unwrap());
    assert!(parse_signal("[8,1]").unwrap() < parse_signal("[9]").unwrap());
    assert!(parse_signal("[8,1]").unwrap() < parse_signal("9").unwrap());

    assert!(parse_signal("[1,1,3,1,1]").unwrap() < parse_signal("[1,1,5,1,1]").unwrap());
    assert!(parse_signal("[9]").unwrap() > parse_signal("[[8,7,6]]").unwrap());
    assert!(parse_signal("[[8,7,6]]").unwrap() < parse_signal("[9]").unwrap());
    assert!(parse_signal("[[[]]]").unwrap() > parse_signal("[[]]").unwrap());
    assert!(parse_signal("[[]]").unwrap() <= parse_signal("[[]]").unwrap());
}

#[test]
fn test_solve1() {
    const INPUT: &str = include_str!("../inputs/day13_test.txt");
    let parsed = parse(INPUT);
    let solution1 = solve1(&parsed);
    assert_eq!(solution1, 13);
}

#[test]
fn test_solve2() {
    const INPUT: &str = include_str!("../inputs/day13_test.txt");
    let mut parsed = parse2(INPUT);
    let solution = solve2(&mut parsed);
    assert_eq!(solution, 140);
}
