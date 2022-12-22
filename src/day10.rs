use num::abs;

#[derive(Clone, Debug)]
enum I {
    Addx(i64),
    Noop,
}
type State = i64;

fn main() {
    const INPUT: &str = include_str!("../inputs/day10.txt");
    let parsed = parse(INPUT);
    let solution1 = solve1(parsed.clone());
    let pixels = crt_pixels(parsed);

    println!("Solution 1: {:}", solution1);
    draw_crt(pixels);
}

fn solve1(is: Vec<I>) -> i64 {
    let initial = 1;
    let binding = run(initial, is);
    let states: Vec<&State> = binding.iter().collect();

    let strengths = states
        .iter()
        .enumerate()
        .map(|(i, s)| (i + 1) as i64 * (*s));

    strengths.skip(19).step_by(40).sum()
}

fn crt_pixels(is: Vec<I>) -> Vec<char> {
    let initial = 1;
    let binding = run(initial, is);
    let states: Vec<&State> = binding.iter().collect();

    states
        .iter()
        .enumerate()
        .map(|(i, s)| {
            if abs(*s - i as i64 % 40) <= 1 {
                '#'
            } else {
                '.'
            }
        })
        .collect()
}

fn draw_crt(pixels: Vec<char>) {
    for row in pixels.as_slice().chunks(40) {
        println!("{:}", row.iter().cloned().collect::<String>());
    }
}

fn parse(s: &str) -> Vec<I> {
    s.lines().map(parse_line).collect()
}

fn parse_line(s: &str) -> I {
    if s == "noop" {
        I::Noop
    } else {
        I::Addx(s.split(' ').nth(1).unwrap().parse().unwrap())
    }
}

fn do_step(s: State, i: I) -> Vec<State> {
    println!("state: {:?}, i: {:?}", s, i);
    match i {
        I::Noop => vec![s],
        I::Addx(x) => vec![s, s + x],
    }
}

fn run(initial_s: State, is: Vec<I>) -> Vec<State> {
    let mut ret = vec![initial_s];
    let mut run_states = is
        .iter()
        .scan(initial_s, |s, i| {
            let next_states = do_step(*s, i.clone());
            *s = *next_states.last().unwrap();
            Some(next_states)
        })
        .flatten()
        .collect();

    ret.append(&mut run_states);
    ret
}

#[test]
fn test_solve1() {
    const INPUT: &str = include_str!("../inputs/day10_test.txt");
    let parsed = parse(INPUT);
    let solution1 = solve1(parsed);

    assert_eq!(solution1, 13140);
}
