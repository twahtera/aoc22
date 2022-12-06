type State = Vec<Vec<char>>;
type Move = (usize, usize, usize); // n, from, to

fn main() {
    const INPUT: &str = include_str!("../inputs/day5.txt");
    let input = parse_input(INPUT);
    println!(
        "Solution 1: {:}",
        solve1(input.clone()).into_iter().collect::<String>()
    );
    println!(
        "Solution 2: {:}",
        solve2(input).into_iter().collect::<String>()
    );
}

fn solve1((state, moves): (State, Vec<Move>)) -> Vec<char> {
    let final_state: State = moves.iter().fold(state, |s, &m| do_move(s, m));

    final_state
        .iter()
        .filter_map(|s| s.last())
        .copied()
        .collect()
}

fn do_move(mut state: State, (n, from, to): Move) -> State {
    for _ in 0..n {
        let top = state[from - 1].pop().unwrap();
        state[to - 1].push(top);
    }

    state
}

fn parse_input(s: &str) -> (State, Vec<Move>) {
    let sections: Vec<&str> = s.split("\n\n").collect();

    (parse_state(sections[0]), parse_moves(sections[1]))
}

fn parse_state(s: &str) -> State {
    let ls: Vec<&str> = s.lines().rev().collect();

    let parsed_lines: Vec<_> = ls[1..].iter().map(|l| parse_state_line(l)).collect();

    let mut state: State = vec![vec![]; parsed_lines[0].len()];

    parsed_lines.into_iter().for_each(|line| {
        line.iter().enumerate().into_iter().for_each(|(i, c)| {
            if *c != ' ' {
                state[i].push(*c)
            }
        });
    });

    state
}

fn parse_state_line(s: &str) -> Vec<char> {
    s.chars().skip(1).step_by(4).collect()
}

fn parse_moves(s: &str) -> Vec<Move> {
    s.lines().map(|l| parse_move(l)).collect()
}

fn parse_move(s: &str) -> Move {
    let v: Vec<&str> = s.split(' ').collect();

    let (n, to, from) = (
        v[1].parse().unwrap(),
        v[3].parse().unwrap(),
        v[5].parse().unwrap(),
    );
    (n, to, from)
}

// star 2:
fn solve2((state, moves): (State, Vec<Move>)) -> Vec<char> {
    let final_state: State = moves.iter().fold(state, |s, &m| do_move2(s, m));

    final_state
        .iter()
        .filter_map(|s| s.last())
        .copied()
        .collect()
}

fn do_move2(mut state: State, (n, from, to): Move) -> State {
    let new_from_len = state[from - 1].len() - n;
    let mut crates = state[from - 1].split_off(new_from_len);
    state[to - 1].append(&mut crates);

    state
}

#[test]
fn test_parse_move() {
    assert_eq!(parse_move("move 1 from 2 to 1"), (1, 2, 1));
}

#[test]
fn test_parse_state_line() {
    assert_eq!(parse_state_line("[Z] [M] [P]"), vec!['Z', 'M', 'P']);
}

#[test]
fn test_parse_state() {
    let parsed = parse_state("    [D]    \n[N] [C]    \n[Z] [M] [P]\n 1   2   3 ");
    let expected = vec![vec!['Z', 'N'], vec!['M', 'C', 'D'], vec!['P']];
    assert_eq!(parsed, expected)
}

#[test]
fn test_solve1() {
    const INPUT: &str = include_str!("../inputs/day5_test.txt");
    let input = parse_input(INPUT);
    assert_eq!(solve1(input), vec!['C', 'M', 'Z']);
}

#[test]
fn test_solve2() {
    const INPUT: &str = include_str!("../inputs/day5_test.txt");
    let input = parse_input(INPUT);
    assert_eq!(solve2(input), vec!['M', 'C', 'D']);
}
