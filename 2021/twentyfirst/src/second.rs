use std::env;
use std::io::{self};

const CIRCLE_SIZE: u64 = 10;
const DICE_ROLLS_PER_TURN: u64 = 3;

const DETERM_DICE_SIDES: u64 = 100;
const WIN_SCORE_PART1: u64 = 1000;

const WIN_SCORE_PART2: u64 = 21;

fn add_arrays(first: [u64;2], second: [u64;2]) -> [u64;2] {
    let mut new: [u64;2] = [0;2];
    new[0] = first[0] + second[0];
    new[1] = first[1] + second[1];
    new
}

#[macro_use] extern crate lazy_static;
lazy_static! {
    static ref ROLLS: Vec<[u64; 3]> = {
        let mut poss: Vec<[u64; 3]> = Vec::new();
        for d1 in 1..=3 {
            for d2 in 1..=3 {
                for d3 in 1..=3 {
                    poss.push([d1, d2, d3]);
                }
            }
        }
        poss
    };
}

#[macro_use] extern crate cached;
#[cached]
fn play_from(starting: [u64;2], scores: [u64;2], turn: usize, rolls: Option<[u64; 3]>) -> [u64; 2] = {
    let mut wins: [u64; 2] = [0; 2];

    // Initialize game
    let mut pos:   [u64; 2] = starting;
    let mut score: [u64; 2] = scores;

    // Play next move first
    match rolls {
        Some(roll) => {
            pos[turn] = (pos[turn] + roll[0]+roll[1]+roll[2] - 1) % CIRCLE_SIZE + 1;
            score[turn] += pos[turn];
            if score[turn] >= WIN_SCORE_PART2 {
                wins[turn] += 1;
                return wins;
            }
        },
        _ => {},
    }

    // Keep playing
    let player = if turn == 0 { 1 } else { 0 };
    for roll in ROLLS.iter() {
        wins = add_arrays(wins,play_from(pos,score, player, Some(*roll)));
    }
    wins
}

fn play_part1(starting: [u64;2]) -> u64 {
    // Play game
    let mut pos:   [u64; 2] = starting;
    let mut score: [u64; 2] = [0; 2];
    let mut rolls = 0;
    let mut dice = 0;
    'play: loop {

        // Player Loop
        for player in 0..2 {
            let mut roll = 0;
            for _ in 0..DICE_ROLLS_PER_TURN {
                dice += 1;
                if dice > DETERM_DICE_SIDES { dice = 1; }
                roll += dice;
            }
            pos[player] = (pos[player] + roll - 1) % CIRCLE_SIZE + 1;
            score[player] += pos[player];
            rolls += DICE_ROLLS_PER_TURN;
            if score[player] >= WIN_SCORE_PART1 { break 'play; }
        }
    }
    let loser = score.iter().min().unwrap();
    loser * rolls
}

fn solve(input: &str) -> io::Result<()> {
    // Input
    let input_str = std::fs::read_to_string(input).unwrap();
    let input_str = input_str.trim();
    
    // Starting positions
    let starting = input_str
        .split("\n")
        .map(|l| l.chars().last().unwrap().to_digit(10).unwrap())
        .collect::<Vec<_>>();

    let starting: [u64; 2] = [starting[0].into(), starting[1].into()];

    // Part 1
    let part1 = play_part1(starting.into());
    println!("Part 1: {}", part1); // 1004670

    // Part 2
    let part2 = play_from(starting, [0;2], 1, None); // start with player 1 because rolls=None (effectively starts with player 0)
    let part2 = part2.iter().max().unwrap();
    println!("Part 2: {}", part2); // 492043106122795

    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    solve(&filename).unwrap();
}