use itertools::iproduct;

type Forest = Vec<Vec<i64>>;

fn main() {
    const INPUT: &str = include_str!("../inputs/day8.txt");
    let parsed = parse(INPUT);
    let solution1 = solve1(&parsed);
    let solution2 = solve2(&parsed);

    println!("Solution 1: {:}", solution1);
    println!("Solution 2: {:}", solution2);
}

fn parse(s: &str) -> Forest {
    s.lines().map(parse_line).collect()
}

fn parse_line(s: &str) -> Vec<i64> {
    s.chars().map(|c| c.to_string().parse().unwrap()).collect()
}

fn visible(f: &Forest, x: usize, y: usize) -> bool {
    let h: i64 = f[y][x];

    let height = f.len();
    let width = f[0].len();

    if x == 0 || y == 0 || x == width - 1 || y == height - 1 {
        return true;
    }

    let from_left = (0..x).all(|i| f[y][i] < h);
    let from_right = ((x + 1)..width).all(|i| f[y][i] < h);

    let from_top = (0..y).all(|i| f[i][x] < h);
    let from_bottom = (y + 1..height).all(|i| f[i][x] < h);

    from_left || from_right || from_top || from_bottom
}

fn solve1(f: &Forest) -> i64 {
    let height = f.len();
    let width = f[0].len();

    iproduct!((0..height), (0..width))
        .filter(|(y, x)| visible(f, *x, *y))
        .count() as i64
}

fn scenic_score(f: &Forest, x: usize, y: usize) -> i64 {
    let h: i64 = f[y][x];

    let height = f.len();
    let width = f[0].len();

    println!("h: {:}", h);

    // unwrap_or ei palauta unwrap_or koko homman pituus...
    let left = x;
    let right = width - x - 1;
    let top = y;
    let bottom = height - y - 1;

    let to_left = (0..x)
        .rev()
        .position(|i| f[y][i] >= h)
        .map(|x| x + 1)
        .unwrap_or(left);
    let to_right = ((x + 1)..width)
        .position(|i| f[y][i] >= h)
        .map(|x| x + 1)
        .unwrap_or(right);

    let sln = (y + 1..height).position(|i| f[i][x] >= h);
    let asd: Vec<_> = ((x + 1)..width).collect();
    println!("is: {:?} heights: {:?}", asd, sln);

    let to_top = (0..y)
        .rev()
        .position(|i| f[i][x] >= h)
        .map(|x| x + 1)
        .unwrap_or(top);
    let to_bottom = (y + 1..height)
        .position(|i| f[i][x] >= h)
        .map(|x| x + 1)
        .unwrap_or(bottom);

    println!("{:}, {:}, {:}, {:}", to_left, to_right, to_top, to_bottom);

    (to_left * to_right * to_top * to_bottom) as i64
}

fn solve2(f: &Forest) -> i64 {
    let height = f.len();
    let width = f[0].len();

    iproduct!((0..height), (0..width))
        .map(|(y, x)| scenic_score(f, x, y))
        .max()
        .unwrap_or(0) as i64
}

#[test]
fn test_parse() {
    const INPUT: &str = include_str!("../inputs/day8_test.txt");
    let parsed = parse(INPUT);

    let expected = vec![
        vec![3, 0, 3, 7, 3],
        vec![2, 5, 5, 1, 2],
        vec![6, 5, 3, 3, 2],
        vec![3, 3, 5, 4, 9],
        vec![3, 5, 3, 9, 0],
    ];

    assert_eq!(parsed, expected);
}

#[test]
fn test_visible() {
    const INPUT: &str = include_str!("../inputs/day8_test.txt");
    let f = parse(INPUT);

    assert!(visible(&f, 1, 1));
    assert!(!visible(&f, 2, 2));
    assert!(visible(&f, 0, 0));

    assert!(visible(&f, 1, 1));
    assert!(visible(&f, 2, 1));
    assert!(visible(&f, 1, 2));
    assert!(visible(&f, 3, 2));
    assert!(visible(&f, 2, 3));
}

#[test]
fn test_solve1() {
    const INPUT: &str = include_str!("../inputs/day8_test.txt");
    let parsed = parse(INPUT);
    let solution = solve1(&parsed);

    assert_eq!(solution, 21);
}

#[test]
fn test_scenic_height() {
    const INPUT: &str = include_str!("../inputs/day8_test.txt");
    let parsed = parse(INPUT);
    let score = scenic_score(&parsed, 2, 1);

    assert_eq!(score, 4);

    assert_eq!(scenic_score(&parsed, 2, 3), 8);
}

#[test]
fn test_solve2() {
    const INPUT: &str = include_str!("../inputs/day8_test.txt");
    let parsed = parse(INPUT);
    let solution = solve2(&parsed);

    assert_eq!(solution, 8);
}
