fn main() {
    println!("Hello, world!");
    let input = include_str!("input.txt");
    let actual = part_1(input);
    println!("{}", actual);
}

fn part_1(input: &str)-> usize {
    input.lines().map(|x|  {
        let numbers: Vec<_> = x.chars().filter(|x| x.is_numeric()).collect();
        let first = match numbers.first() {
            Some(expr) => expr,
            None => panic!("should have got a number"),
        };
        let first_and_last_numbers = format!("{}{}", first, numbers.last().unwrap());
        first_and_last_numbers.parse::<usize>().unwrap()
    }).sum()
}

#[test]
fn some_thing() {
    let input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
    let actual = part_1(input);
    let expected = 142;
    assert_eq!(actual, expected);
}
