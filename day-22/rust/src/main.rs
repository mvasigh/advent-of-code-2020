use std::cmp;
use std::collections::{HashSet, VecDeque};
use std::fs;

#[derive(PartialEq, Eq)]
enum Player {
    One,
    Two
}

type GameResult = (Player, VecDeque<i32>);

fn get_starter_decks() -> (VecDeque<i32>, VecDeque<i32>) {
    let decks = fs::read_to_string("data.txt")
        .expect("Could not read input file")
        .trim()
        .split("\n\n")
        .map(|c| {
            c.split("\n")
                .skip(1)
                .map(|v| v.parse::<i32>().unwrap())
                .collect::<VecDeque<i32>>()
        })
        .collect::<Vec<VecDeque<i32>>>();

    (decks[0].to_owned(), decks[1].to_owned())
}

fn play_round(p1: &mut VecDeque<i32>, p2: &mut VecDeque<i32>) {
    let p1_card = p1.pop_front().expect("No cards left for player 1!");
    let p2_card = p2.pop_front().expect("No cards left for player 2!");
    let winner = if p2_card > p1_card { p2 } else { p1 };

    winner.push_back(cmp::max(p1_card, p2_card));
    winner.push_back(cmp::min(p1_card, p2_card));
}

fn create_round_signature(p1: &VecDeque<i32>, p2: &VecDeque<i32>) -> String {
    let mut signature = String::new();
    for deck in [p1, p2].iter() {
        signature.push_str(
            &deck
                .iter()
                .map(|v| v.to_string())
                .collect::<Vec<String>>()
                .join(""),
        );
        signature.push('|');
    }
    signature
}

fn create_subdeck(deck: &VecDeque<i32>, num: &i32) -> VecDeque<i32> {
    deck.to_owned()
        .iter()
        .take(*num as usize)
        .map(|el| el.to_owned())
        .collect::<VecDeque<i32>>()
}

fn play_game(mut p1: VecDeque<i32>, mut p2: VecDeque<i32>, depth: i32) -> GameResult {
    let mut prev_hands: HashSet<String> = HashSet::new();
    let mut _round = 1;

    // println!("\n=== Game {} ===", depth);

    while !p1.is_empty() && !p2.is_empty() {
        // println!("\n-- Round {} (Game {}) --", _round, depth);
        // println!("Player 1's deck: {:?}", p1);
        // println!("Player 2's deck: {:?}", p2);
        let signature = create_round_signature(&p1, &p2);
        
        // If this combo of hands has been seen before, p1 is the winner
        if prev_hands.contains(&signature) {
            // println!("Player 1 wins round {} of game {}!", _round, depth);
            return (Player::One, p1);
        }

        prev_hands.insert(signature);

        // If not, draw cards as normal
        let p1_card = p1.pop_front().expect("No cards left for player 1!");
        let p2_card = p2.pop_front().expect("No cards left for player 2!");
        // println!("Player 1 plays: {}", p1_card);
        // println!("Player 2 plays: {}", p2_card);

        let winner = if p1.len() >= p1_card as usize && p2.len() >= p2_card as usize {
            // println!("Playing a subgame to determine the winner...");
            let slice_p1 = create_subdeck(&p1, &p1_card);
            let slice_p2 = create_subdeck(&p2, &p2_card);


            let winner = match play_game(slice_p1, slice_p2, depth + 1).0 {
                Player::One => {
                    // println!("Player 1 wins round {} of game {}!", _round, depth);
                    (Player::One, &mut p1)
                },
                Player::Two => {
                    // println!("Player 2 wins round {} of game {}!", _round, depth);
                    (Player::Two, &mut p2)
                }
            };

            // println!("\n...anyway, back to game {}", depth);

            winner
        } else {
            if p2_card > p1_card {
                // println!("Player 2 wins round {} of game {}!", _round, depth);
                (Player::Two, &mut p2)
            } else {
                // println!("Player 1 wins round {} of game {}!", _round, depth);
                (Player::One, &mut p1)
            }
        };

        let winner_deck = winner.1;

        if winner.0 == Player::One {
            winner_deck.push_back(p1_card);
            winner_deck.push_back(p2_card);
        } else if winner.0 == Player::Two {
            winner_deck.push_back(p2_card);
            winner_deck.push_back(p1_card);
        }

        _round += 1;
    }

    let winner = if p1.len() > p2.len() { (Player::One, p1) } else { (Player::Two, p2) };

    winner
}

fn part_one() -> i32 {
    let (mut p1, mut p2) = get_starter_decks();
    let mut _rounds = 0;

    while p1.len() > 0 && p2.len() > 0 {
        play_round(&mut p1, &mut p2);
        _rounds += 1;
    }

    let winner = if p1.len() > p2.len() { p1 } else { p2 };

    let total_cards = winner.len();
    winner
        .iter()
        .enumerate()
        .fold(0, |acc, (i, curr)| acc + ((total_cards - i) as i32 * curr))
}

fn part_two() -> i32 {
    let (p1, p2) = get_starter_decks();
    let (_winning_player, winning_deck) = play_game(p1, p2, 1);

    let total_cards = winning_deck.len();
    winning_deck
        .iter()
        .enumerate()
        .fold(0, |acc, (i, curr)| acc + ((total_cards - i) as i32 * curr))
}

fn main() {
    println!("Part 1: {}", part_one());
    println!("Part 2: {}", part_two());
}
