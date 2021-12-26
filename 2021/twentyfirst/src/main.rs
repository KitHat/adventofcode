fn main() {
    println!("test 1 {}", solve_practice(3, 10));
    let mut game = Game {
        first_pos: 3,
        second_pos: 10,
        first_score: 0,
        second_score: 0,
        turn: true,
        universes: 1
    };
    println!("test 1 {}", solve_dirac(&mut game));
}

#[derive(Clone, Debug)]
struct Game {
    first_pos: usize,
    second_pos: usize,
    first_score: usize,
    second_score: usize,
    turn: bool,
    universes: u128
}

fn universes() -> [(usize, u128); 7] {
    [
        (3, 1),
        (4, 3),
        (5, 6),
        (6, 7),
        (7, 6),
        (8, 3),
        (9, 1)
    ]
}

fn solve_dirac(game: &mut Game) -> u128 {
    let mut first_wins = 0;
    let mut second_wins = 0;
    let mut games: Vec<Game> = vec![];
    games.push(game.clone());
    while let Some(game) = games.pop() {
        for outcome in universes() {
            let mut new_game = game.clone();
            if game.turn {
                new_game.first_pos += outcome.0;
                if new_game.first_pos > 10 {
                    new_game.first_pos -= 10;
                }
                new_game.universes *= outcome.1;
                new_game.first_score += new_game.first_pos;
                if new_game.first_score >= 21 {
                    first_wins += new_game.universes;
                } else {
                    new_game.turn = false;
                    games.push(new_game);
                }
            } else {
                new_game.second_pos += outcome.0;
                if new_game.second_pos > 10 {
                    new_game.second_pos -= 10;
                }
                new_game.universes *= outcome.1;
                new_game.second_score += new_game.second_pos;
                if new_game.second_score >= 21 {
                    second_wins += new_game.universes;
                } else {
                    new_game.turn = true;
                    games.push(new_game);
                }
            }
        }
    }
    println!("first {} second {}", first_wins, second_wins);
    if first_wins > second_wins {first_wins} else {second_wins}
}

fn solve_practice(first: usize, second: usize) -> usize {
    let mut turn = true;
    let mut first_pos: usize = first;
    let mut second_pos = second;
    let mut first_score = 0;
    let mut second_score = 0;
    let mut i: usize = 0;
    let mut hundreds = 0;
    while first_score < 1000 && second_score < 1000 {
        if turn {
            i=next_dieroll(i, &mut hundreds);
            first_pos += i;
            i=next_dieroll(i, &mut hundreds);
            first_pos += i;
            i=next_dieroll(i, &mut hundreds);
            first_pos += i;
            while first_pos > 10 {
                first_pos -= 10;
            }
            first_score += first_pos;
            // println!("{} {}", turn, first_score);
        } else {
            i=next_dieroll(i, &mut hundreds);
            second_pos += i;
            i=next_dieroll(i, &mut hundreds);
            second_pos += i;
            i=next_dieroll(i, &mut hundreds);
            second_pos += i;
            while second_pos > 10 {
                second_pos -= 10;
            }
            second_score += second_pos;
            // println!("{} {}", turn, second_score);
        }
        turn = !turn;
    }
    println!("hundreds {} i {}", hundreds, i);
    (i+hundreds*100)*(if first_score >= 1000 { second_score} else {first_score})
}

fn next_dieroll(prev: usize, hundreds: &mut usize) -> usize {
    if prev == 100 {
        *hundreds += 1;
        1
    } else {
        prev + 1
    }
}
