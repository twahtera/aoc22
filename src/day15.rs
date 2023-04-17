use std::collections::HashSet;

type Point = (i64, i64);

fn main() {
    const INPUT: &str = include_str!("../inputs/day15.txt");
    let parsed = parse(INPUT);

    let solution1 = solve1(2000000, parsed.clone());
    println!("Solution 1: {:}", solution1);

    let solution2 = solve2(4000000, parsed);
    println!("Solution 2: {:}", solution2);
}

fn parse_point(s: &str) -> Point {
    let parts: Vec<&str> = s.split(", ").collect();
    (
        parts[0].split('=').nth(1).unwrap().parse().unwrap(),
        parts[1].split('=').nth(1).unwrap().parse().unwrap(),
    )
}

fn parse_line(s: &str) -> (Point, Point) {
    let parts: Vec<&str> = s.split(": closest beacon is at ").collect();
    (parse_point(&parts[0][10..]), parse_point(parts[1]))
}

fn parse(s: &str) -> Vec<(Point, Point)> {
    s.lines().map(parse_line).collect()
}

fn manhattan((a1, a2): Point, (b1, b2): Point) -> i64 {
    (a1 - b1).abs() + (a2 - b2).abs()
}

// return the endpoints of an excluded area on the line y
fn line_exclusions(y: i64, scanner: Point, beacon: Point) -> (i64, i64) {
    let dist = manhattan(scanner, beacon);
    let excl_width = dist - (y - scanner.1).abs();

    (scanner.0 - excl_width, scanner.0 + excl_width)
}

fn solve1(y: i64, pairs: Vec<(Point, Point)>) -> i64 {
    let mut exclusions: HashSet<i64> = pairs
        .iter()
        .map(|(s, b)| line_exclusions(y, *s, *b))
        .flat_map(|(start, end)| start..end + 1)
        .collect();

    let beacons_on_line = pairs.iter().map(|(_, b)| b).filter(|(_, by)| *by == y);
    beacons_on_line.for_each(|(bx, _)| {
        let _ = exclusions.remove(bx);
    });

    let mut vals: Vec<&i64> = exclusions.iter().collect();
    vals.sort();

    exclusions.len() as i64
}

// Since we know that there can only be one point not covered by the
// scanners, it needs to be on a point right outside the range of some
// scanner and it's enough to check those.
fn solve2(search_size: usize, pairs: Vec<(Point, Point)>) -> i64 {
    let coverages: Vec<(Point, i64)> = pairs.iter().map(|(s, b)| (*s, manhattan(*s, *b))).collect();

    let mut points = coverages
        .iter()
        .flat_map(|(p, c)| around(search_size as i64, *p, *c));

    let found = points.find(|(x, y)| !is_covered((*x as i64, *y as i64), &coverages));

    let uw = found.unwrap();
    (uw.0 as i64) * 4000000 + uw.1 as i64
}

fn is_covered(p: Point, coverages: &Vec<(Point, i64)>) -> bool {
    for (s, r) in coverages {
        if manhattan(p, *s) <= *r {
            return true;
        }
    }
    false
}

// Return the points around a scanner's scan radius
fn around(search_size: i64, (x, y): Point, r: i64) -> Vec<Point> {
    (0..r + 1)
        .flat_map(|i| {
            [
                (x + -r - 1 + i, y + i),
                (x + i, y + -r - 1 + i),
                (x + r + 1 - i, y + i),
                (x + -i, y + r + 1 - i),
            ]
        })
        .filter(|(a, b)| *a >= 0 && *a <= search_size && *b >= 0 && *b <= search_size)
        .collect()
}

#[test]
fn test_parse_line() {
    assert_eq!(
        parse_line("Sensor at x=2, y=18: closest beacon is at x=-2, y=15"),
        ((2, 18), (-2, 15))
    );
}

#[test]
fn test_line_exclusion() {
    let res = line_exclusions(10, (8, 7), (2, 10));
    assert_eq!(res, (2, 14));
}

#[test]
fn test_solve1() {
    const INPUT: &str = include_str!("../inputs/day15_test.txt");
    let parsed = parse(INPUT);

    let solution = solve1(10, parsed);
    assert_eq!(solution, 26);
}

#[test]
fn test_around() {
    assert_eq!(around(100, (10, 10), 5).len(), 4 * 6);
}

#[test]
fn test_solve2() {
    const INPUT: &str = include_str!("../inputs/day15_test.txt");
    let parsed = parse(INPUT);

    let solution = solve2(20, parsed);
    assert_eq!(solution, 56000011);
}
