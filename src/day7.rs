fn main() {}

#[derive(PartialEq, Debug)]
enum AocFile {
    Dir {name: String, children: Vec<AocFile>, size: i64},
    File{name: String, size: i64},
}

fn parse_file(s: &str) -> Option<AocFile> {
    let parts: Vec<&str> = s.split(' ').collect();
    let name = String::from(*parts.get(1)?);
    let size = (*parts.first()?).parse().ok()?;

    Some(AocFile::File{name, size})
}

fn parse_dir(name: &str, ls: &mut std::str::Lines) -> Option<AocFile> {
    // the next line should be ls, we don't need it but check for sanity
    let ls_line = ls.next();
    println!("parsing new dir {:}, {:}", name, ls_line?);
    assert_eq!(ls_line, Some("$ ls"));


    let mut children: Vec<AocFile> = vec![];
    loop {
        let line = ls.next();
        match line {
            None => break,
            Some(l) => {
                println!("parsing line: {:}", l);
                if l.trim() == "$ cd .." {
                    println!("up dir");
                    break;
                }
                if l[..3] == *"dir" {
                    println!("skip dir listing");
                    continue;
                }
                if l[..4] == *"$ cd" {
                    let next_name = l.split(' ').nth(1);
                    children.push(parse_dir(next_name?, ls)?);
                }
                let file = parse_file(l)?;
                children.push(file);
            }
        }
    }
    let size = children.iter().map(|c| file_size(&c)).sum();
    Some(AocFile::Dir{name: String::from(name), children, size })
}

fn file_size(f: &AocFile) -> i64 {
    match f {
        AocFile::File{size, ..} => *size,
        AocFile::Dir{size, ..} => *size
    }
}

fn flatten_dir(f : &AocFile) -> Vec<&AocFile> {
    match f {
        AocFile::File{..}  => vec![f],
        AocFile::Dir{children,..} => {
            let mut this_list = vec![f];
            let flat_children  = children.iter().flat_map(|c| flatten_dir(c));
            this_list.extend(flat_children);

            this_list
        }
    }
}




#[test]
fn test_parse_file() {
    assert_eq!(
        parse_file("4060174 j"),
        Some(AocFile::File{name: String::from("j"), size: 4060174})
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
            println!("s: \"{:}\"", s);
            assert!(s == "$cd ..")
        },
        None => ()
    }
}

#[test]
fn test_parse_dir() {
    const INPUT: &str = include_str!("../inputs/day7_test.txt");
    let mut lines: std::str::Lines = INPUT.lines();
    lines.next();
    let parsed = parse_dir("/", &mut lines);
    if let Some(AocFile::Dir{children, size, ..}) = parsed {
        assert_eq!(children.len(), 4);
        assert_eq!(size, 48380581);
    }

}

#[test]
fn test_flatten() {
    const INPUT: &str = include_str!("../inputs/day7_test.txt");
    let mut lines: std::str::Lines = INPUT.lines();
    lines.next();
    let parsed = parse_dir("/", &mut lines);
    let flat = parsed.as_ref().map(|d| flatten_dir(d));

    println!("parsed: {:?}")

    assert_eq!(flat.map(|v| v.len()), Some(12));


}
