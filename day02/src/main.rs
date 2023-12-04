fn main() {
    println!("Hello, world!");
    let part_1_input = include_str!("input.txt");
    let part_1_answer = part_1(part_1_input);
    println!("{part_1_answer}");
}

fn part_1(input: &str) -> usize {
    let answer = input.lines().filter_map(possible_game).sum();
    answer
}

fn possible_game(line: &str) -> Option<usize> {
    let (game, rounds) = line.split_once(':').unwrap();
    let round_count = rounds.split(';').count();
    let possible_rounds = rounds.split(';').filter(|x| is_possible_round(x)).count();
    if round_count == possible_rounds {
        let (_, game_id_str) = game.split_once(' ').unwrap();
        let game_id: usize = game_id_str.parse().unwrap();
        return Some(game_id);
    }
    None
}

fn is_possible_round(round: &str) -> bool {
    let count = round.split(',').count();
    let possible = round.split(',').filter(|x| is_possible_hand(x)).count();
    count == possible
}

fn is_possible_hand(hand: &str) -> bool {
    let (count_str, colour) = hand.trim().split_once(' ').unwrap();
    let count: usize = count_str.parse().unwrap();

    match colour {
        "red" => count <= 12,
        "green" => count <= 13,
        "blue" => count <= 14,
        _ => panic!("should not get here"),
    }
}

fn is_possible_round2(round: &str) -> bool {
    let count = round.split(',').count();
    let possible = round.split(',').filter(|x| is_possible_hand(x)).count();
    count == possible
}

fn is_possible_hand2(hand: &str) -> Option<usize> {
    let (count_str, colour) = hand.trim().split_once(' ').unwrap();
    let count: usize = count_str.parse().unwrap();

    match colour {
        "red" => {
            if count <= 12 {
                Some(count)
            } else {
                None
            }
        }
        "green" => {
            if count <= 13 {
                Some(count)
            } else {
                None
            }
        }
        "blue" => {
            if count <= 14 {
                Some(count)
            } else {
                None
            }
        }
        _ => None,
    }
}

#[test]
fn part1_works() {
    let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
    let actual = part_1(input);
    let expected = 8;
    assert_eq!(actual, expected);
}
