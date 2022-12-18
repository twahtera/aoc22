struct Monkey<'a> {
    items: Vec<i64>,
    op: Box<dyn Fn(i64) -> i64 + 'a>,
    test: Box<dyn Fn(i64) -> usize + 'a>,
    inspects: i64,
}

fn main() {
    const INPUT: &str = include_str!("../inputs/day11.txt");
    let parsed = parse(INPUT);
    let solution1 = solve1(parsed);

    let parsed2 = parse(INPUT);
    let mod_by = get_mod(INPUT);
    let solution2 = solve2(parsed2, mod_by);

    println!("Solution 1: {:}", solution1);
    println!("Solution 2: {:}", solution2);
}

fn solve1(mut monkeys: Vec<Monkey>) -> i64 {
    for _ in 0..20 {
        play_round(&mut monkeys);
    }

    let mut inspects: Vec<i64> = monkeys.iter().map(|m| m.inspects).collect();
    inspects.sort_by(|a, b| b.cmp(a));

    inspects[0] * inspects[1]
}

fn solve2(mut monkeys: Vec<Monkey>, mod_by: i64) -> i64 {
    for _ in 0..10000 {
        play_round2(&mut monkeys, mod_by);
    }

    let mut inspects: Vec<i64> = monkeys.iter().map(|m| m.inspects).collect();
    inspects.sort_by(|a, b| b.cmp(a));

    inspects[0] * inspects[1]
}

fn parse(s: &str) -> Vec<Monkey> {
    s.split("\n\n").map(parse_monkey).collect()
}

fn parse_monkey(s: &str) -> Monkey {
    let ls: Vec<&str> = s.lines().collect();
    let items: Vec<i64> = ls[1][18..]
        .split(", ")
        .map(|str| str.parse().unwrap())
        .collect();
    let op: Box<dyn Fn(i64) -> i64> = parse_operation(ls[2]);
    let test_lines: Vec<&str> = ls[3..6].to_vec();
    let test = parse_test(&test_lines);

    Monkey {
        items,
        op,
        test,
        inspects: 0,
    }
}

fn parse_operation(s: &str) -> Box<dyn Fn(i64) -> i64 + '_> {
    let op_parts: Vec<&str> = s[19..].split(' ').collect();

    let var1 = parse_num(op_parts[0]);
    let var2 = parse_num(op_parts[2]);
    let func = parse_numop(op_parts[1]);

    Box::new(move |a| func(var1(a), var2(a)))
}

fn parse_numop(s: &str) -> fn(i64, i64) -> i64 {
    match s {
        "+" => |a, b| a + b,
        "*" => |a, b| a * b,
        _ => panic!("Failed to parse operation \"{:}\"", s),
    }
}

fn parse_num(s: &'_ str) -> Box<dyn Fn(i64) -> i64 + '_> {
    match s {
        "old" => Box::new(|a| a),
        num => Box::new(|_| num.parse().unwrap()),
    }
}

fn parse_test<'a>(s: &'_ [&str]) -> Box<dyn Fn(i64) -> usize + 'a> {
    let div_by: i64 = s[0].split(' ').last().unwrap().parse().unwrap();
    let true_target: usize = s[1].split(' ').last().unwrap().parse().unwrap();
    let false_target: usize = s[2].split(' ').last().unwrap().parse().unwrap();

    Box::new(move |a| {
        if a % div_by == 0 {
            true_target
        } else {
            false_target
        }
    })
}

fn play_round(monkeys: &mut Vec<Monkey>) {
    for i in 0..monkeys.len() {
        for j in 0..monkeys[i].items.len() {
            let new_worry = (monkeys[i].op)(monkeys[i].items[j]) / 3;
            let throw_to = (monkeys[i].test)(new_worry);
            monkeys[throw_to].items.push(new_worry);
            monkeys[i].inspects += 1;
        }
        monkeys[i].items = vec![];
    }
}

fn play_round2(monkeys: &mut Vec<Monkey>, mod_by: i64) {
    for i in 0..monkeys.len() {
        for j in 0..monkeys[i].items.len() {
            let new_worry = (monkeys[i].op)(monkeys[i].items[j]) % mod_by;
            let throw_to = (monkeys[i].test)(new_worry);
            monkeys[throw_to].items.push(new_worry);
            monkeys[i].inspects += 1;
        }
        monkeys[i].items = vec![];
    }
}

fn get_mod(s: &str) -> i64 {
    let monkeys_s = s.split("\n\n");
    let mut ret = 1;
    for ms in monkeys_s {
        let div_line: &str = ms.lines().nth(3).unwrap();
        let div: i64 = div_line[21..].parse().unwrap();
        ret *= div;
    }
    ret
}

#[test]
fn test_monkey_operation() {
    let op = parse_operation("  Operation: new = old * 19");
    assert_eq!(op(2), 2 * 19);
}

#[test]
fn test_monkey_test() {
    let test = parse_test(&[
        "  Test: divisible by 23",
        "    If true: throw to monkey 2",
        "    If false: throw to monkey 3",
    ]);
    assert_eq!(test(46), 2);
    assert_eq!(test(44), 3);
}

#[test]
fn test_solve1() {
    const INPUT: &str = include_str!("../inputs/day11_test.txt");
    let parsed = parse(INPUT);
    let solution1 = solve1(parsed);
    assert_eq!(solution1, 10605);
}

#[test]
fn test_solve2() {
    const INPUT: &str = include_str!("../inputs/day11_test.txt");
    let parsed = parse(INPUT);
    let mod_by = get_mod(INPUT);
    let solution2 = solve2(parsed, mod_by);
    assert_eq!(solution2, 2713310158);
}
