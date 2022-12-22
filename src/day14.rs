use std::collections::HashMap;

#[derive(Clone)]
struct Map {
    height: i64,
    m: HashMap<Point, char>,
}

type Point = (i64, i64);

fn main() {
    const INPUT: &str = include_str!("../inputs/day14.txt");
    let parsed = parse(INPUT);
    let mut m = build_map(parsed);
    let solution1 = solve1(&mut m.clone());

    let solution2 = solve2(&mut m);

    println!("Solution 1: {:}", solution1);
    println!("Solution 2: {:}", solution2);
}

fn segment_points((a1, a2): Point, (b1, b2): Point) -> Vec<Point> {
    if a1 == b1 {
        if a2 < b2 {
            (a2..b2 + 1).map(|snd| (a1, snd)).collect()
        } else {
            (b2..a2 + 1).map(|snd| (a1, snd)).collect()
        }
    } else {
        if a1 < b1 {
            (a1..b1 + 1).map(|fst| (fst, a2)).collect()
        } else {
            (b1..a1 + 1).map(|fst| (fst, a2)).collect()
        }
    }
}

fn wall_points(ps: &Vec<Point>) -> Vec<Point> {
    ps.windows(2)
        .flat_map(|w| segment_points(w[0], w[1]))
        .collect()
}

fn parse_line(s: &str) -> Vec<Point> {
    s.split(" -> ").map(parse_point).collect()
}

fn parse_point(s: &str) -> Point {
    let parts: Vec<&str> = s.split(',').collect();
    (parts[0].parse().unwrap(), parts[1].parse().unwrap())
}

fn parse(s: &str) -> Vec<Vec<Point>> {
    s.lines().map(parse_line).collect()
}

fn build_map(walls: Vec<Vec<Point>>) -> Map {
    let points: Vec<Point> = walls.iter().flat_map(wall_points).collect();
    let height = points.iter().map(|p| p.1).max().unwrap_or(0);
    let m: HashMap<Point, char> = points.iter().map(|p| (*p, '#')).collect();

    Map { height, m }
}

fn drop_sand_1(map: &Map, (x, y): Point) -> Option<Point> {
    let candidates = [(x, y + 1), (x - 1, y + 1), (x + 1, y + 1)];
    candidates.into_iter().find(|c| !map.m.contains_key(c))
}

// Spawn a unit of sand on the map and drop it as far as it will go
fn drop_sand(map: &Map) -> Option<Point> {
    let mut new_sand = (500, 0);
    if map.m.contains_key(&new_sand) {
        return None;
    };

    while new_sand.1 <= map.height {
        match drop_sand_1(map, new_sand) {
            Some(next_sand) => new_sand = next_sand,
            None => return Some(new_sand),
        }
    }
    None
}

fn play(map: &mut Map) {
    while let Some(next_sand) = drop_sand(map) {
        map.m.insert(next_sand, 'o');
    }
}

fn solve1(map: &mut Map) -> usize {
    play(map);
    map.m.iter().filter(|(_, v)| **v == 'o').count()
}

fn solve2(map: &mut Map) -> usize {
    map.height += 2;
    (0..1000).for_each(|x| {
        map.m.insert((x, map.height), '#');
    });

    play(map);
    map.m.iter().filter(|(_, v)| **v == 'o').count()
}

#[test]
fn test_parse() {
    assert_eq!(parse_point("498,4"), (498, 4));
    assert_eq!(
        parse_line("498,4 -> 498,6 -> 496,6"),
        vec![(498, 4), (498, 6), (496, 6)]
    );
}

#[test]
fn test_solve1() {
    const INPUT: &str = include_str!("../inputs/day14_test.txt");
    let parsed = parse(INPUT);
    let mut m = build_map(parsed);
    let solution = solve1(&mut m);
    assert_eq!(solution, 24);
}

#[test]
fn test_solve2() {
    const INPUT: &str = include_str!("../inputs/day14_test.txt");
    let parsed = parse(INPUT);
    let mut m = build_map(parsed);
    let solution = solve2(&mut m);
    assert_eq!(solution, 93);
}
