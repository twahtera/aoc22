fn main() {
    const INPUT: &str = include_str!("../inputs/day7.txt");
    let mut lines: std::str::Lines = INPUT.lines();
    lines.next();
    let parsed = parse_dir("/", &mut lines).unwrap();
    let solution1 = solve1(&parsed);
    let solution2 = solve2(&parsed);

    println!("Solution 1: {:}", solution1);
    println!("Solution 2: {:}", solution2);
}

#[derive(PartialEq, Debug)]
enum AocFile {
    Dir {
        name: String,
        children: Vec<AocFile>,
        size: i64,
    },
    File {
        name: String,
        size: i64,
    },
}

fn parse_file(s: &str) -> Option<AocFile> {
    let parts: Vec<&str> = s.split(' ').collect();
    let name = String::from(*parts.get(1)?);
    let size = (*parts.first()?).parse().ok()?;

    Some(AocFile::File { name, size })
}

fn parse_dir(name: &str, ls: &mut std::str::Lines) -> Option<AocFile> {
    // the next line should be ls, we don't need it but check for sanity
    let ls_line = ls.next();
    assert_eq!(ls_line, Some("$ ls"));

    let mut children: Vec<AocFile> = vec![];
    loop {
        let line = ls.next();
        match line {
            None => break,
            Some(l) => {
                if l.trim() == "$ cd .." {
                    break;
                }
                if l[..3] == *"dir" {
                    continue;
                }
                if l[..4] == *"$ cd" {
                    let next_name = l.split(' ').nth(2);
                    children.push(parse_dir(next_name?, ls)?);
                }
                if let Some(file) = parse_file(l) {
                    children.push(file);
                }
            }
        }
    }

    let size = children.iter().map(file_size).sum();
    Some(AocFile::Dir {
        name: String::from(name),
        children,
        size,
    })
}

fn file_size(f: &AocFile) -> i64 {
    match f {
        AocFile::File { size, .. } => *size,
        AocFile::Dir { size, .. } => *size,
    }
}

fn flatten_dir(f: &AocFile) -> Vec<&AocFile> {
    match f {
        AocFile::File { .. } => vec![f],
        AocFile::Dir { children, .. } => {
            let mut this_list = vec![f];
            let flat_children = children.iter().flat_map(flatten_dir);
            this_list.extend(flat_children);

            this_list
        }
    }
}

fn solve1(root: &AocFile) -> i64 {
    flatten_dir(&root)
        .iter()
        .filter(|f| is_dir(f))
        .map(|d| file_size(d))
        .filter(|size| size <= &100000)
        .sum()
}

fn is_dir(f: &AocFile) -> bool {
    matches!(f, AocFile::Dir { .. })
}

fn solve2(root: &AocFile) -> i64 {
    let total_space: i64 = 70000000;
    let used = file_size(root);
    let free = total_space - used;
    let to_free = 30000000 - free;

    let binding = flatten_dir(root);
    let big_enough = binding
        .iter()
        .filter(|f| is_dir(f))
        .map(|d| file_size(d))
        .filter(|size| size >= &to_free);

    big_enough.min().unwrap_or(0)
}

#[test]
fn test_parse_file() {
    assert_eq!(
        parse_file("4060174 j"),
        Some(AocFile::File {
            name: String::from("j"),
            size: 4060174
        })
    );

    assert_eq!(parse_file("slnt"), None)
}

#[test]
fn test_sanity() {
    let a = "abc";
    let b = "abc";
    assert!(a == b);

    let mut l = "$cd ..\n".lines();
    match l.next() {
        Some(s) => {
            assert!(s == "$cd ..")
        }
        None => (),
    }
}

#[test]
fn test_parse_dir() {
    const INPUT: &str = include_str!("../inputs/day7_test.txt");
    let mut lines: std::str::Lines = INPUT.lines();
    lines.next();
    let parsed = parse_dir("/", &mut lines);
    if let Some(AocFile::Dir { children, size, .. }) = parsed {
        assert_eq!(children.len(), 4);
        assert_eq!(size, 48381165);
    } else {
        assert!(false);
    }
}

#[test]
fn test_flatten() {
    const INPUT: &str = include_str!("../inputs/day7_test.txt");
    let mut lines: std::str::Lines = INPUT.lines();
    lines.next();
    let parsed = parse_dir("/", &mut lines);

    let flat = parsed.as_ref().map(flatten_dir);

    assert_eq!(flat.map(|v| v.len()), Some(14));
}

#[test]
fn test_solve1() {
    const INPUT: &str = include_str!("../inputs/day7_test.txt");
    let mut lines: std::str::Lines = INPUT.lines();
    lines.next();
    let parsed = parse_dir("/", &mut lines).unwrap();
    let solution = solve1(&parsed);

    assert_eq!(solution, 95437);
}

#[test]
fn test_solve2() {
    const INPUT: &str = include_str!("../inputs/day7_test.txt");
    let mut lines: std::str::Lines = INPUT.lines();
    lines.next();
    let parsed = parse_dir("/", &mut lines).unwrap();
    let solution = solve2(&parsed);

    assert_eq!(solution, 24933642);
}
