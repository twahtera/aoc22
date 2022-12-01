fn main() {
    println!("Hello, day1!");
}

fn solve(mut a: i64, mut b: i64) -> i64 {
    a + b
}

#[test]
fn testDay1() {
  assert_eq!(solve(1,2), 1+2)
}

