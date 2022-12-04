type Input = Vec<Vec<i64>>;

fn main() {
    const INPUT: &str = include_str!("../inputs/day1.txt");
    let input = parse_input(INPUT).unwrap();

    let solution = solve(&input);

    println!("The solution for problem 1 is {:?}", solution);

    let solution2 = solve2(&input);
    println!("The solution for problem 2 is {:?}", solution2);
}

fn solve(calorie_lists: &Input) -> i64 {
    let elf_calories: Vec<i64> = calorie_lists.iter().map(|list| list.iter().sum()).collect();
    let maxval = elf_calories.iter().max().unwrap_or(&0);

    *maxval
}

fn solve2(calorie_lists: &Input) -> i64 {
    let mut elf_calories: Vec<i64> = calorie_lists.iter().map(|list| list.iter().sum()).collect();
    elf_calories.sort();

    elf_calories[elf_calories.len() - 3..elf_calories.len()]
        .iter()
        .sum()
}

fn parse_input(input_str: &str) -> Result<Input, std::io::Error> {
    let mut output: Input = vec![];
    let mut cur_group = vec![];

    for line in input_str.lines() {
        if line.is_empty() {
            output.push(cur_group);
            cur_group = vec![];
        } else {
            let num = line.parse().unwrap();
            cur_group.push(num);
        }
    }
    output.push(cur_group);

    Ok(output)
}

#[test]
fn test_parser() {
    assert_eq!(parse_input("").unwrap(), vec![vec![]]);
    assert_eq!(parse_input("1\n2").unwrap(), vec![vec![1, 2]]);
    assert_eq!(parse_input("1\n2\n\n3").unwrap(), vec![vec![1, 2], vec![3]]);
}

#[test]
fn test_day1() {
    const TEST_INPUT: &str = include_str!("../inputs/day1_test1.txt");
    let input = parse_input(TEST_INPUT).unwrap();

    assert_eq!(solve(&input), 24000);
    assert_eq!(solve2(&input), 45000);
}
