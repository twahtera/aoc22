use std::collections::HashSet;

fn main() {
    const INPUT: &str = include_str!("../inputs/day3.txt");

    let input: Vec<&str> = INPUT.lines().collect();

    println!("Solution 1: {:?}", solve1(input.as_slice()));
    println!("Solution 2: {:?}", solve2(input.as_slice()));
}

fn solve1(sacks: &[&str]) -> i64 {
    sacks
        .iter()
        .map(|s| rucksack_priority(s.chars().collect()))
        .sum()
}

fn solve2(sacks: &[&str]) -> i64 {
    let char_vecs: Vec<Vec<char>> = sacks.to_vec().iter().map(|s| s.chars().collect()).collect();
    let groups = char_vecs.chunks(3);
    let priorities = groups.map(|g| group_priority(g.to_vec()));
    priorities.sum()
}

fn priority(c: char) -> i64 {
    if c.is_uppercase() {
        c as i64 - 38
    } else {
        c as i64 - 96
    }
}

fn find_common(a: Vec<char>, b: Vec<char>) -> Option<char> {
    let set_a: HashSet<char> = a.into_iter().collect();
    let set_b: HashSet<char> = b.into_iter().collect();

    let common: Vec<&char> = set_a.intersection(&set_b).collect();

    Some(*common[0])
}

fn group_priority(group: Vec<Vec<char>>) -> i64 {
    let a = &group[0];
    let b = &group[1];
    let c = &group[2];

    find_common_3(a, b, c).map(priority).unwrap_or(0)
}

fn find_common_3(a: &Vec<char>, b: &Vec<char>, c: &Vec<char>) -> Option<char> {
    let set2: HashSet<char> = b.clone().into_iter().collect();
    let set3: HashSet<char> = c.clone().into_iter().collect();
    let sets = [&set2, &set3];

    let common: Vec<&char> = a
        .iter()
        .filter(|k| sets.iter().all(|s| s.contains(k)))
        .collect();

    Some(*common[0])
}

fn split_bags(items: Vec<char>) -> (Vec<char>, Vec<char>) {
    (
        items[0..items.len() / 2].to_vec(),
        items[items.len() / 2..items.len()].to_vec(),
    )
}

fn rucksack_priority(items: Vec<char>) -> i64 {
    let (a, b) = split_bags(items);

    find_common(a, b).map(priority).unwrap_or(0)
}

#[test]
fn test_priority() {
    assert_eq!(priority('p'), 16);
    assert_eq!(priority('L'), 38);
    assert_eq!(priority('P'), 42);
    assert_eq!(priority('v'), 22);
    assert_eq!(priority('t'), 20);
    assert_eq!(priority('s'), 19);
}

#[test]
fn test_solve1() {
    const TEST_INPUT: &str = include_str!("../inputs/day3_test.txt");

    let input: Vec<&str> = TEST_INPUT.lines().collect();
    assert_eq!(solve1(input.as_slice()), 157);
}

#[test]
fn test_find_common_3() {
    assert_eq!(
        find_common_3(
            &"vJrwpWtwJgWrhcsFMMfFFhFp".chars().collect(),
            &"jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL".chars().collect(),
            &"PmmdzqPrVvPwwTWBwg".chars().collect(),
        ),
        Some('r'),
    );
}
