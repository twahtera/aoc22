use itertools::iproduct;

#[derive(Clone)]
struct Map {
    map: Vec<Vec<i64>>,
    width: usize,
    height: usize,
    start: Point,
    goal: Point,
}

type Point = (usize, usize);

fn main() {
    const INPUT: &str = include_str!("../inputs/day12.txt");
    let parsed = parse(INPUT);
    let solution1 = solve1(&parsed);
    let solution2 = solve2(&parsed);

    println!("Solution 1: {:}", solution1);
    println!("Solution 2: {:}", solution2);
}

fn solve1(m: &Map) -> i64 {
    let dist = dists(m);

    dist[m.goal.0 + m.goal.1 * m.width]
}

fn solve2(m: &Map) -> i64 {
    let low_points =
        iproduct!((0..m.width), (0..m.height)).filter(|(x, y)| m.map[*y][*x] == 'a' as i64);

    low_points
        .map(|p| {
            let mut new_map = (*m).clone();
            new_map.start = p;
            solve1(&new_map)
        })
        .min()
        .unwrap()
}

fn dists(m: &Map) -> Vec<i64> {
    // distance vector, implicitly: i64::MAX if not visited
    let mut dist = vec![std::i64::MAX; m.width * m.height];
    dist[m.start.0 + m.start.1 * m.width] = 0;

    let mut cur;
    let mut front = vec![m.start];

    while !front.is_empty() {
        cur = front.pop().unwrap(); // just tested to be unempty. probably a cleaner way to do this!

        for (nx, ny) in neighbours(m, cur) {
            let alt = 1 + dist[cur.0 + cur.1 * m.width];
            if alt < dist[nx + ny * m.width] {
                dist[nx + ny * m.width] = alt;
                front.push((nx, ny))
            }
        }
    }

    dist
}

fn parse(s: &str) -> Map {
    let ls = s.lines();
    let map: Vec<Vec<i64>> = ls.map(|l| l.chars().map(char_height).collect()).collect();
    let width = map[0].len();
    let height = map.len();

    let start = i_to_point(s.find('S').unwrap(), width);
    let goal = i_to_point(s.find('E').unwrap(), width);

    Map {
        map,
        width,
        height,
        start,
        goal,
    }
}

fn char_height(c: char) -> i64 {
    match c {
        'S' => 'a' as i64,
        'E' => 'z' as i64,
        c => c as i64,
    }
}

// get the map coordinate from index in string
fn i_to_point(i: usize, width: usize) -> Point {
    // i = x  + y * (width+1)
    let x = i % (width + 1);
    let y = i / (width + 1);
    (x, y)
}

// The points around a point that are at most one higher
fn neighbours(m: &Map, (x, y): Point) -> Vec<Point> {
    let cur_height = m.map[y][x];
    around(m, (x, y))
        .iter()
        .filter(|(a, b)| m.map[*b][*a] - cur_height <= 1)
        .copied()
        .collect()
}

fn around(m: &Map, (x, y): Point) -> Vec<Point> {
    vec![(0, -1), (-1, 0), (1, 0), (0, 1)]
        .iter()
        .filter(|p| **p != (0, 0))
        .filter(|(a, b)| {
            (x as i64 + a >= 0)
                && ((x as i64 + a) < m.width as i64)
                && (y as i64 + b >= 0)
                && ((m.height as i64) > (y as i64 + b))
        })
        .map(|(a, b)| ((x as i64 + a) as usize, (y as i64 + *b) as usize))
        .collect()
}

#[test]
fn test_parse() {
    const INPUT: &str = include_str!("../inputs/day12_test.txt");
    let map = parse(INPUT);

    assert_eq!(map.width, 8);
    assert_eq!(map.height, 5);
    assert_eq!(map.start, (0, 0));
    assert_eq!(map.goal, (5, 2));
}

#[test]
fn test_around() {
    const INPUT: &str = include_str!("../inputs/day12_test.txt");
    let map = parse(INPUT);

    assert_eq!(around(&map, (0, 0)).len(), 2);
    assert_eq!(around(&map, (1, 0)).len(), 3);
    assert_eq!(around(&map, (1, 1)).len(), 4);
    assert_eq!(around(&map, (4, 4)).len(), 3);
    assert_eq!(around(&map, (7, 4)).len(), 2);
}

#[test]
fn test_solve1() {
    const INPUT: &str = include_str!("../inputs/day12_test.txt");
    let parsed = parse(INPUT);
    let solution1 = solve1(&parsed);
    assert_eq!(solution1, 31);
}

#[test]
fn test_solve2() {
    const INPUT: &str = include_str!("../inputs/day12_test.txt");
    let parsed = parse(INPUT);
    let solution1 = solve2(&parsed);
    assert_eq!(solution1, 29);
}
