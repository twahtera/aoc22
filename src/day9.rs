use num::{abs, signum};
use std::cmp;
use std::collections::HashSet;

#[derive(Clone, Debug)]
enum Step {
    L,
    R,
    U,
    D,
}

type Coord = (i64, i64); // x, y
type State = (Coord, Coord); // head, tail
type State10 = [Coord; 10];

fn main() {
    const INPUT: &str = include_str!("../inputs/day9.txt");
    let parsed = parse(INPUT);
    let solution1 = solve1(parsed.clone());
    let solution2 = solve2(parsed);

    println!("Solution 1: {:}", solution1);
    println!("Solution 2: {:}", solution2);
}

fn solve1(steps: Vec<Step>) -> i64 {
    let initial = ((0, 0), (0, 0));
    let states = run(initial, steps);
    let tails: HashSet<(i64, i64)> = states.into_iter().map(|(_, t)| t).collect();

    tails.len() as i64
}

fn solve2(steps: Vec<Step>) -> i64 {
    let initial = [(0, 0); 10];
    let states = run10(initial, steps);
    let tails: HashSet<_> = states.into_iter().map(|s| s[9]).collect();

    tails.len() as i64
}

fn run(initial: State, steps: Vec<Step>) -> Vec<State> {
    steps
        .iter()
        .scan(initial, |state, step| {
            let next_step = do_step(*state, step.clone());
            *state = next_step;
            Some(next_step)
        })
        .collect()
}

fn parse(s: &str) -> Vec<Step> {
    let ls = s.lines();
    ls.flat_map(parse_line).collect()
}

fn parse_line(s: &str) -> Vec<Step> {
    let parts: Vec<&str> = s.split(' ').collect();
    let step = match parts[0] {
        "L" => Step::L,
        "R" => Step::R,
        "U" => Step::U,
        "D" => Step::D,
        _ => panic!("Failed parsing line {:?}", s),
    };
    let count = parts[1].parse().unwrap();
    vec![step; count]
}

fn do_step(state: State, step: Step) -> State {
    let (old_head, old_tail) = state;
    let head = move_head(old_head, step);
    let tail = move_tail(head, old_tail);

    (head, tail)
}

fn do_step10(state: State10, step: Step) -> State10 {
    let mut new_state = [(0, 0); 10];
    new_state[0] = move_head(state[0], step);

    for i in 1..10 {
        new_state[i] = move_tail(new_state[i - 1], state[i]);
    }
    new_state
}

fn run10(initial: State10, steps: Vec<Step>) -> Vec<State10> {
    steps
        .iter()
        .scan(initial, |state, step| {
            let next_step = do_step10(*state, step.clone());
            *state = next_step;
            Some(next_step)
        })
        .collect()
}

fn move_head((hx, hy): (i64, i64), s: Step) -> (i64, i64) {
    match s {
        Step::L => (hx - 1, hy),
        Step::R => (hx + 1, hy),
        Step::U => (hx, hy + 1),
        Step::D => (hx, hy - 1),
    }
}

fn move_tail((hx, hy): (i64, i64), (tx, ty): (i64, i64)) -> (i64, i64) {
    let xdiff = abs(tx - hx);
    let ydiff = abs(ty - hy);

    let max_diff = cmp::max(xdiff, ydiff);

    if max_diff < 2 {
        (tx, ty)
    } else if xdiff != 0 && ydiff != 0 {
        (move_num_towards(tx, hx), move_num_towards(ty, hy))
    } else if xdiff != 0 {
        (move_num_towards(tx, hx), ty)
    } else if ydiff != 0 {
        (tx, move_num_towards(ty, hy))
    } else {
        panic!("Illegal state, head: {:?}, tail: {:?}", (hx, hy), (tx, ty))
    }
}

fn move_num_towards(a: i64, b: i64) -> i64 {
    a + signum(b - a)
}

#[test]
fn test_move_num_towards() {
    assert_eq!(move_num_towards(0, 2), 1);
    assert_eq!(move_num_towards(4, 2), 3);
}

#[test]
fn test_move_head() {
    assert_eq!(move_head((0, 0), Step::R), (1, 0));
}

#[test]
fn test_do_step() {
    assert_eq!(do_step(((0, 0), (0, 0)), Step::R), ((1, 0), (0, 0)));
    assert_eq!(do_step(((1, 0), (0, 0)), Step::R), ((2, 0), (1, 0)));
}

#[test]
fn test_sanity() {
    let a = [1, 2, 3];
    let scan: Vec<i64> = a
        .iter()
        .scan(1, |state, i| {
            println!("state: {:}, i: {:}", state, i);
            let next_state = *state + i;
            *state = next_state;
            Some(next_state)
        })
        .collect();
    assert_eq!(scan, vec![2, 4, 7]);
}

#[test]
fn test_solve1() {
    const INPUT: &str = include_str!("../inputs/day9_test.txt");
    let parsed = parse(INPUT);
    let solution1 = solve1(parsed);
    assert_eq!(solution1, 13);
}

#[test]
fn test_solve2() {
    const INPUT: &str = include_str!("../inputs/day9_test2.txt");
    let parsed = parse(INPUT);
    let solution = solve2(parsed);
    assert_eq!(solution, 36);
}
