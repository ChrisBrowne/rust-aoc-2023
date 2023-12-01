fn main() {
    println!("Hello, world!");
    let input = include_str!("input.txt");
    let part_1_answer = part_1(input);
    println!("{}", part_1_answer);
    let part_2_answer = part_2(input);
    println!("{}", part_2_answer);
}

fn part_1(input: &str) -> usize {
    input
        .lines()
        .map(|x| {
            let numbers: Vec<_> = x.chars().filter(|x| x.is_numeric()).collect();
            let first = match numbers.first() {
                Some(expr) => expr,
                None => panic!("should have got a number"),
            };
            let first_and_last_numbers = format!("{}{}", first, numbers.last().unwrap());
            first_and_last_numbers.parse::<usize>().unwrap()
        })
        .sum()
}

fn replace_and_parse(input: &str) -> usize {
    input
        .replace("one", "1")
        .replace("two", "2")
        .replace("three", "3")
        .replace("four", "4")
        .replace("five", "5")
        .replace("six", "6")
        .replace("seven", "7")
        .replace("eight", "8")
        .replace("nine", "9")
        .parse()
        .unwrap()
}

fn part_2(input: &str) -> usize {
    let items: Vec<[&str; 2]> = vec![
        ["1", "one"],
        ["2", "two"],
        ["3", "three"],
        ["4", "four"],
        ["5", "five"],
        ["6", "six"],
        ["7", "seven"],
        ["8", "eight"],
        ["9", "nine"],
    ];
    input
        .lines()
        .map(|line| {
            let first_items = items
                .iter()
                .flatten()
                .map(|x| {
                    let index = line.find(x);
                    (x, index)
                })
                .filter(|x| x.1.is_some());
            let mut vec: Vec<_> = first_items.collect();
            vec.sort_by(|a, b| a.1.unwrap().cmp(&b.1.unwrap()));

            let first = replace_and_parse(vec.first().unwrap().0);
            let last_items = items
                .iter()
                .flatten()
                .map(|x| {
                    let index = line.rfind(x);
                    (x, index)
                })
                .filter(|x| x.1.is_some());
            let mut vec: Vec<_> = last_items.collect();
            vec.sort_by(|a, b| a.1.unwrap().cmp(&b.1.unwrap()));

            let last = replace_and_parse(vec.last().unwrap().0);

            let first_and_last_numbers = format!("{}{}", first, last);
            first_and_last_numbers.parse::<usize>().unwrap()
        })
        .sum()
}

#[test]
fn part_1_test() {
    let input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
    let actual = part_1(input);
    let expected = 142;
    assert_eq!(actual, expected);
}

#[test]
fn part_2_test() {
    let input = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";
    let actual = part_2(input);
    let expected = 281;
    assert_eq!(actual, expected);
}
