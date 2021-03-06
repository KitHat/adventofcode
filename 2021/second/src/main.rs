use serde_derive::Deserialize;

fn main() {
    let test: Vec<Movement> = serde_json::from_str(&input_test()).unwrap();
    println!("test: {}", calc_result(&test));
    let source: Vec<Movement> = serde_json::from_str(&input_source()).unwrap();
    println!("source: {}", calc_result(&source));
}

fn calc_result(movements: &Vec<Movement>) -> u32 {
    let ( _, hor, vert) = movements.iter().fold((0, 0, 0), |(aim, hor, vert), next| {
        match next.dir {
            Direction::UP => (aim - next.num, hor, vert),
            Direction::DOWN => (aim + next.num, hor, vert),
            Direction::FORWARD => (aim, hor + next.num, vert + next.num*aim)
        }
    });
    hor*vert
}

#[derive(Deserialize)]
enum Direction{
    UP,
    DOWN,
    FORWARD
}

#[derive(Deserialize)]
struct Movement {
    dir: Direction,
    num: u32
}

fn input_test() -> String {
    r#"[{"dir":"FORWARD", "num":5},
    {"dir":"DOWN", "num":5},
    {"dir":"FORWARD", "num":8},
    {"dir":"UP", "num":3},
    {"dir":"DOWN", "num":8},
    {"dir":"FORWARD", "num":2}]"#.to_string()
}

fn input_source() -> String {
    r#"[{"dir":"FORWARD", "num":6},
    {"dir":"FORWARD", "num":8},
    {"dir":"DOWN", "num":6},
    {"dir":"DOWN", "num":6},
    {"dir":"FORWARD", "num":9},
    {"dir":"DOWN", "num":7},
    {"dir":"DOWN", "num":3},
    {"dir":"FORWARD", "num":4},
    {"dir":"FORWARD", "num":7},
    {"dir":"UP", "num":3},
    {"dir":"FORWARD", "num":9},
    {"dir":"UP", "num":4},
    {"dir":"FORWARD", "num":4},
    {"dir":"DOWN", "num":3},
    {"dir":"UP", "num":2},
    {"dir":"FORWARD", "num":2},
    {"dir":"DOWN", "num":4},
    {"dir":"FORWARD", "num":9},
    {"dir":"UP", "num":4},
    {"dir":"FORWARD", "num":8},
    {"dir":"UP", "num":9},
    {"dir":"FORWARD", "num":3},
    {"dir":"DOWN", "num":7},
    {"dir":"DOWN", "num":4},
    {"dir":"DOWN", "num":7},
    {"dir":"UP", "num":5},
    {"dir":"DOWN", "num":5},
    {"dir":"DOWN", "num":5},
    {"dir":"FORWARD", "num":5},
    {"dir":"UP", "num":9},
    {"dir":"DOWN", "num":5},
    {"dir":"DOWN", "num":3},
    {"dir":"UP", "num":3},
    {"dir":"FORWARD", "num":2},
    {"dir":"FORWARD", "num":2},
    {"dir":"DOWN", "num":2},
    {"dir":"FORWARD", "num":1},
    {"dir":"DOWN", "num":4},
    {"dir":"DOWN", "num":1},
    {"dir":"DOWN", "num":6},
    {"dir":"DOWN", "num":9},
    {"dir":"FORWARD", "num":4},
    {"dir":"FORWARD", "num":7},
    {"dir":"DOWN", "num":4},
    {"dir":"UP", "num":7},
    {"dir":"FORWARD", "num":8},
    {"dir":"DOWN", "num":3},
    {"dir":"DOWN", "num":6},
    {"dir":"DOWN", "num":5},
    {"dir":"DOWN", "num":2},
    {"dir":"DOWN", "num":1},
    {"dir":"UP", "num":9},
    {"dir":"FORWARD", "num":2},
    {"dir":"FORWARD", "num":3},
    {"dir":"UP", "num":3},
    {"dir":"FORWARD", "num":5},
    {"dir":"DOWN", "num":4},
    {"dir":"UP", "num":3},
    {"dir":"UP", "num":9},
    {"dir":"FORWARD", "num":6},
    {"dir":"UP", "num":5},
    {"dir":"UP", "num":8},
    {"dir":"UP", "num":8},
    {"dir":"FORWARD", "num":6},
    {"dir":"UP", "num":1},
    {"dir":"FORWARD", "num":4},
    {"dir":"DOWN", "num":4},
    {"dir":"FORWARD", "num":9},
    {"dir":"DOWN", "num":9},
    {"dir":"DOWN", "num":7},
    {"dir":"FORWARD", "num":5},
    {"dir":"DOWN", "num":7},
    {"dir":"FORWARD", "num":7},
    {"dir":"UP", "num":2},
    {"dir":"DOWN", "num":9},
    {"dir":"DOWN", "num":2},
    {"dir":"FORWARD", "num":8},
    {"dir":"FORWARD", "num":9},
    {"dir":"DOWN", "num":4},
    {"dir":"DOWN", "num":4},
    {"dir":"DOWN", "num":7},
    {"dir":"DOWN", "num":4},
    {"dir":"DOWN", "num":8},
    {"dir":"FORWARD", "num":4},
    {"dir":"DOWN", "num":9},
    {"dir":"DOWN", "num":6},
    {"dir":"FORWARD", "num":8},
    {"dir":"DOWN", "num":8},
    {"dir":"DOWN", "num":8},
    {"dir":"FORWARD", "num":7},
    {"dir":"UP", "num":9},
    {"dir":"FORWARD", "num":7},
    {"dir":"UP", "num":3},
    {"dir":"DOWN", "num":5},
    {"dir":"FORWARD", "num":9},
    {"dir":"DOWN", "num":7},
    {"dir":"DOWN", "num":5},
    {"dir":"DOWN", "num":8},
    {"dir":"FORWARD", "num":5},
    {"dir":"DOWN", "num":1},
    {"dir":"DOWN", "num":5},
    {"dir":"DOWN", "num":5},
    {"dir":"FORWARD", "num":8},
    {"dir":"DOWN", "num":8},
    {"dir":"UP", "num":8},
    {"dir":"DOWN", "num":6},
    {"dir":"DOWN", "num":3},
    {"dir":"FORWARD", "num":1},
    {"dir":"DOWN", "num":2},
    {"dir":"DOWN", "num":3},
    {"dir":"UP", "num":4},
    {"dir":"FORWARD", "num":8},
    {"dir":"DOWN", "num":6},
    {"dir":"UP", "num":8},
    {"dir":"FORWARD", "num":2},
    {"dir":"FORWARD", "num":8},
    {"dir":"FORWARD", "num":9},
    {"dir":"FORWARD", "num":5},
    {"dir":"FORWARD", "num":5},
    {"dir":"FORWARD", "num":2},
    {"dir":"FORWARD", "num":1},
    {"dir":"DOWN", "num":8},
    {"dir":"FORWARD", "num":8},
    {"dir":"FORWARD", "num":9},
    {"dir":"UP", "num":8},
    {"dir":"UP", "num":8},
    {"dir":"DOWN", "num":7},
    {"dir":"DOWN", "num":5},
    {"dir":"FORWARD", "num":1},
    {"dir":"FORWARD", "num":1},
    {"dir":"UP", "num":9},
    {"dir":"UP", "num":3},
    {"dir":"FORWARD", "num":5},
    {"dir":"DOWN", "num":8},
    {"dir":"DOWN", "num":4},
    {"dir":"UP", "num":2},
    {"dir":"DOWN", "num":7},
    {"dir":"DOWN", "num":4},
    {"dir":"DOWN", "num":3},
    {"dir":"FORWARD", "num":3},
    {"dir":"FORWARD", "num":7},
    {"dir":"FORWARD", "num":3},
    {"dir":"FORWARD", "num":1},
    {"dir":"FORWARD", "num":8},
    {"dir":"DOWN", "num":9},
    {"dir":"UP", "num":4},
    {"dir":"DOWN", "num":8},
    {"dir":"DOWN", "num":3},
    {"dir":"DOWN", "num":9},
    {"dir":"UP", "num":8},
    {"dir":"UP", "num":6},
    {"dir":"UP", "num":9},
    {"dir":"UP", "num":2},
    {"dir":"DOWN", "num":9},
    {"dir":"DOWN", "num":9},
    {"dir":"DOWN", "num":6},
    {"dir":"UP", "num":3},
    {"dir":"DOWN", "num":4},
    {"dir":"DOWN", "num":3},
    {"dir":"DOWN", "num":9},
    {"dir":"FORWARD", "num":2},
    {"dir":"FORWARD", "num":2},
    {"dir":"FORWARD", "num":7},
    {"dir":"FORWARD", "num":9},
    {"dir":"UP", "num":5},
    {"dir":"UP", "num":8},
    {"dir":"FORWARD", "num":8},
    {"dir":"DOWN", "num":8},
    {"dir":"UP", "num":7},
    {"dir":"FORWARD", "num":4},
    {"dir":"UP", "num":7},
    {"dir":"UP", "num":7},
    {"dir":"DOWN", "num":9},
    {"dir":"DOWN", "num":7},
    {"dir":"DOWN", "num":1},
    {"dir":"FORWARD", "num":4},
    {"dir":"FORWARD", "num":6},
    {"dir":"UP", "num":4},
    {"dir":"FORWARD", "num":7},
    {"dir":"UP", "num":4},
    {"dir":"DOWN", "num":6},
    {"dir":"FORWARD", "num":2},
    {"dir":"UP", "num":4},
    {"dir":"FORWARD", "num":2},
    {"dir":"UP", "num":2},
    {"dir":"DOWN", "num":4},
    {"dir":"DOWN", "num":2},
    {"dir":"FORWARD", "num":4},
    {"dir":"FORWARD", "num":8},
    {"dir":"DOWN", "num":9},
    {"dir":"UP", "num":7},
    {"dir":"FORWARD", "num":3},
    {"dir":"DOWN", "num":5},
    {"dir":"DOWN", "num":2},
    {"dir":"DOWN", "num":7},
    {"dir":"UP", "num":9},
    {"dir":"DOWN", "num":2},
    {"dir":"UP", "num":5},
    {"dir":"DOWN", "num":9},
    {"dir":"DOWN", "num":4},
    {"dir":"FORWARD", "num":8},
    {"dir":"FORWARD", "num":9},
    {"dir":"DOWN", "num":7},
    {"dir":"UP", "num":3},
    {"dir":"DOWN", "num":4},
    {"dir":"UP", "num":3},
    {"dir":"DOWN", "num":3},
    {"dir":"DOWN", "num":3},
    {"dir":"UP", "num":5},
    {"dir":"FORWARD", "num":7},
    {"dir":"DOWN", "num":8},
    {"dir":"DOWN", "num":1},
    {"dir":"UP", "num":7},
    {"dir":"FORWARD", "num":8},
    {"dir":"FORWARD", "num":9},
    {"dir":"FORWARD", "num":3},
    {"dir":"FORWARD", "num":1},
    {"dir":"UP", "num":4},
    {"dir":"FORWARD", "num":4},
    {"dir":"DOWN", "num":6},
    {"dir":"FORWARD", "num":3},
    {"dir":"DOWN", "num":9},
    {"dir":"FORWARD", "num":2},
    {"dir":"UP", "num":8},
    {"dir":"DOWN", "num":4},
    {"dir":"FORWARD", "num":7},
    {"dir":"FORWARD", "num":1},
    {"dir":"UP", "num":9},
    {"dir":"FORWARD", "num":6},
    {"dir":"DOWN", "num":1},
    {"dir":"DOWN", "num":8},
    {"dir":"FORWARD", "num":7},
    {"dir":"FORWARD", "num":8},
    {"dir":"FORWARD", "num":2},
    {"dir":"UP", "num":6},
    {"dir":"FORWARD", "num":3},
    {"dir":"FORWARD", "num":2},
    {"dir":"FORWARD", "num":7},
    {"dir":"DOWN", "num":6},
    {"dir":"UP", "num":8},
    {"dir":"DOWN", "num":2},
    {"dir":"DOWN", "num":8},
    {"dir":"FORWARD", "num":6},
    {"dir":"FORWARD", "num":1},
    {"dir":"UP", "num":9},
    {"dir":"DOWN", "num":8},
    {"dir":"DOWN", "num":4},
    {"dir":"FORWARD", "num":3},
    {"dir":"DOWN", "num":2},
    {"dir":"FORWARD", "num":1},
    {"dir":"FORWARD", "num":8},
    {"dir":"DOWN", "num":9},
    {"dir":"DOWN", "num":9},
    {"dir":"FORWARD", "num":3},
    {"dir":"DOWN", "num":5},
    {"dir":"DOWN", "num":5},
    {"dir":"DOWN", "num":1},
    {"dir":"DOWN", "num":3},
    {"dir":"UP", "num":3},
    {"dir":"DOWN", "num":6},
    {"dir":"FORWARD", "num":9},
    {"dir":"FORWARD", "num":7},
    {"dir":"UP", "num":5},
    {"dir":"FORWARD", "num":8},
    {"dir":"UP", "num":1},
    {"dir":"UP", "num":4},
    {"dir":"DOWN", "num":8},
    {"dir":"UP", "num":3},
    {"dir":"FORWARD", "num":1},
    {"dir":"UP", "num":9},
    {"dir":"FORWARD", "num":5},
    {"dir":"DOWN", "num":8},
    {"dir":"DOWN", "num":1},
    {"dir":"UP", "num":3},
    {"dir":"DOWN", "num":1},
    {"dir":"DOWN", "num":7},
    {"dir":"FORWARD", "num":2},
    {"dir":"DOWN", "num":9},
    {"dir":"FORWARD", "num":3},
    {"dir":"DOWN", "num":2},
    {"dir":"UP", "num":5},
    {"dir":"DOWN", "num":5},
    {"dir":"FORWARD", "num":9},
    {"dir":"DOWN", "num":2},
    {"dir":"DOWN", "num":3},
    {"dir":"FORWARD", "num":4},
    {"dir":"FORWARD", "num":2},
    {"dir":"DOWN", "num":5},
    {"dir":"FORWARD", "num":9},
    {"dir":"DOWN", "num":5},
    {"dir":"FORWARD", "num":9},
    {"dir":"DOWN", "num":9},
    {"dir":"DOWN", "num":8},
    {"dir":"FORWARD", "num":8},
    {"dir":"FORWARD", "num":9},
    {"dir":"DOWN", "num":2},
    {"dir":"FORWARD", "num":4},
    {"dir":"FORWARD", "num":9},
    {"dir":"UP", "num":1},
    {"dir":"DOWN", "num":2},
    {"dir":"DOWN", "num":5},
    {"dir":"FORWARD", "num":8},
    {"dir":"FORWARD", "num":5},
    {"dir":"UP", "num":5},
    {"dir":"FORWARD", "num":2},
    {"dir":"UP", "num":8},
    {"dir":"FORWARD", "num":3},
    {"dir":"DOWN", "num":2},
    {"dir":"FORWARD", "num":1},
    {"dir":"FORWARD", "num":7},
    {"dir":"FORWARD", "num":7},
    {"dir":"FORWARD", "num":1},
    {"dir":"UP", "num":9},
    {"dir":"DOWN", "num":5},
    {"dir":"DOWN", "num":3},
    {"dir":"DOWN", "num":7},
    {"dir":"FORWARD", "num":2},
    {"dir":"UP", "num":5},
    {"dir":"FORWARD", "num":1},
    {"dir":"FORWARD", "num":7},
    {"dir":"UP", "num":7},
    {"dir":"DOWN", "num":3},
    {"dir":"UP", "num":5},
    {"dir":"DOWN", "num":1},
    {"dir":"FORWARD", "num":9},
    {"dir":"DOWN", "num":7},
    {"dir":"FORWARD", "num":4},
    {"dir":"FORWARD", "num":5},
    {"dir":"DOWN", "num":1},
    {"dir":"DOWN", "num":5},
    {"dir":"UP", "num":7},
    {"dir":"FORWARD", "num":1},
    {"dir":"DOWN", "num":4},
    {"dir":"DOWN", "num":8},
    {"dir":"DOWN", "num":6},
    {"dir":"FORWARD", "num":8},
    {"dir":"FORWARD", "num":4},
    {"dir":"FORWARD", "num":2},
    {"dir":"UP", "num":8},
    {"dir":"DOWN", "num":5},
    {"dir":"DOWN", "num":2},
    {"dir":"FORWARD", "num":2},
    {"dir":"DOWN", "num":1},
    {"dir":"DOWN", "num":8},
    {"dir":"DOWN", "num":5},
    {"dir":"FORWARD", "num":5},
    {"dir":"FORWARD", "num":8},
    {"dir":"DOWN", "num":9},
    {"dir":"FORWARD", "num":1},
    {"dir":"FORWARD", "num":3},
    {"dir":"DOWN", "num":3},
    {"dir":"DOWN", "num":1},
    {"dir":"DOWN", "num":5},
    {"dir":"FORWARD", "num":6},
    {"dir":"FORWARD", "num":6},
    {"dir":"DOWN", "num":3},
    {"dir":"FORWARD", "num":5},
    {"dir":"FORWARD", "num":8},
    {"dir":"DOWN", "num":9},
    {"dir":"DOWN", "num":9},
    {"dir":"FORWARD", "num":4},
    {"dir":"FORWARD", "num":4},
    {"dir":"FORWARD", "num":1},
    {"dir":"FORWARD", "num":7},
    {"dir":"DOWN", "num":6},
    {"dir":"UP", "num":2},
    {"dir":"DOWN", "num":2},
    {"dir":"UP", "num":6},
    {"dir":"FORWARD", "num":7},
    {"dir":"FORWARD", "num":8},
    {"dir":"UP", "num":3},
    {"dir":"DOWN", "num":7},
    {"dir":"UP", "num":9},
    {"dir":"DOWN", "num":5},
    {"dir":"DOWN", "num":7},
    {"dir":"DOWN", "num":7},
    {"dir":"FORWARD", "num":3},
    {"dir":"DOWN", "num":3},
    {"dir":"UP", "num":9},
    {"dir":"UP", "num":5},
    {"dir":"FORWARD", "num":4},
    {"dir":"DOWN", "num":7},
    {"dir":"DOWN", "num":7},
    {"dir":"FORWARD", "num":1},
    {"dir":"DOWN", "num":1},
    {"dir":"DOWN", "num":2},
    {"dir":"FORWARD", "num":8},
    {"dir":"DOWN", "num":9},
    {"dir":"DOWN", "num":2},
    {"dir":"FORWARD", "num":3},
    {"dir":"FORWARD", "num":5},
    {"dir":"DOWN", "num":4},
    {"dir":"UP", "num":7},
    {"dir":"FORWARD", "num":1},
    {"dir":"FORWARD", "num":4},
    {"dir":"FORWARD", "num":7},
    {"dir":"FORWARD", "num":1},
    {"dir":"UP", "num":1},
    {"dir":"DOWN", "num":6},
    {"dir":"FORWARD", "num":2},
    {"dir":"DOWN", "num":1},
    {"dir":"FORWARD", "num":2},
    {"dir":"FORWARD", "num":7},
    {"dir":"FORWARD", "num":7},
    {"dir":"UP", "num":3},
    {"dir":"DOWN", "num":8},
    {"dir":"FORWARD", "num":6},
    {"dir":"FORWARD", "num":1},
    {"dir":"DOWN", "num":6},
    {"dir":"DOWN", "num":8},
    {"dir":"DOWN", "num":1},
    {"dir":"UP", "num":3},
    {"dir":"UP", "num":6},
    {"dir":"DOWN", "num":6},
    {"dir":"DOWN", "num":1},
    {"dir":"UP", "num":8},
    {"dir":"FORWARD", "num":6},
    {"dir":"UP", "num":6},
    {"dir":"FORWARD", "num":5},
    {"dir":"UP", "num":4},
    {"dir":"UP", "num":7},
    {"dir":"UP", "num":4},
    {"dir":"DOWN", "num":5},
    {"dir":"FORWARD", "num":2},
    {"dir":"FORWARD", "num":7},
    {"dir":"DOWN", "num":1},
    {"dir":"FORWARD", "num":6},
    {"dir":"DOWN", "num":5},
    {"dir":"UP", "num":3},
    {"dir":"UP", "num":4},
    {"dir":"FORWARD", "num":1},
    {"dir":"UP", "num":4},
    {"dir":"FORWARD", "num":7},
    {"dir":"UP", "num":5},
    {"dir":"UP", "num":1},
    {"dir":"UP", "num":2},
    {"dir":"FORWARD", "num":4},
    {"dir":"FORWARD", "num":2},
    {"dir":"FORWARD", "num":7},
    {"dir":"DOWN", "num":6},
    {"dir":"FORWARD", "num":1},
    {"dir":"UP", "num":5},
    {"dir":"UP", "num":2},
    {"dir":"DOWN", "num":8},
    {"dir":"FORWARD", "num":5},
    {"dir":"UP", "num":2},
    {"dir":"DOWN", "num":9},
    {"dir":"UP", "num":4},
    {"dir":"DOWN", "num":2},
    {"dir":"DOWN", "num":1},
    {"dir":"FORWARD", "num":8},
    {"dir":"DOWN", "num":2},
    {"dir":"UP", "num":8},
    {"dir":"DOWN", "num":8},
    {"dir":"UP", "num":7},
    {"dir":"UP", "num":4},
    {"dir":"UP", "num":2},
    {"dir":"FORWARD", "num":8},
    {"dir":"UP", "num":1},
    {"dir":"UP", "num":6},
    {"dir":"FORWARD", "num":8},
    {"dir":"FORWARD", "num":8},
    {"dir":"FORWARD", "num":2},
    {"dir":"DOWN", "num":5},
    {"dir":"UP", "num":2},
    {"dir":"DOWN", "num":1},
    {"dir":"FORWARD", "num":2},
    {"dir":"UP", "num":5},
    {"dir":"FORWARD", "num":4},
    {"dir":"FORWARD", "num":9},
    {"dir":"DOWN", "num":1},
    {"dir":"DOWN", "num":6},
    {"dir":"FORWARD", "num":4},
    {"dir":"DOWN", "num":3},
    {"dir":"DOWN", "num":7},
    {"dir":"DOWN", "num":5},
    {"dir":"DOWN", "num":6},
    {"dir":"UP", "num":6},
    {"dir":"FORWARD", "num":4},
    {"dir":"DOWN", "num":2},
    {"dir":"DOWN", "num":4},
    {"dir":"DOWN", "num":3},
    {"dir":"FORWARD", "num":3},
    {"dir":"FORWARD", "num":2},
    {"dir":"UP", "num":7},
    {"dir":"FORWARD", "num":7},
    {"dir":"FORWARD", "num":6},
    {"dir":"FORWARD", "num":3},
    {"dir":"FORWARD", "num":1},
    {"dir":"FORWARD", "num":8},
    {"dir":"FORWARD", "num":3},
    {"dir":"FORWARD", "num":2},
    {"dir":"FORWARD", "num":3},
    {"dir":"FORWARD", "num":9},
    {"dir":"DOWN", "num":3},
    {"dir":"FORWARD", "num":9},
    {"dir":"UP", "num":9},
    {"dir":"UP", "num":1},
    {"dir":"DOWN", "num":8},
    {"dir":"DOWN", "num":2},
    {"dir":"UP", "num":9},
    {"dir":"DOWN", "num":8},
    {"dir":"FORWARD", "num":5},
    {"dir":"DOWN", "num":8},
    {"dir":"UP", "num":8},
    {"dir":"DOWN", "num":3},
    {"dir":"UP", "num":7},
    {"dir":"DOWN", "num":3},
    {"dir":"DOWN", "num":7},
    {"dir":"UP", "num":9},
    {"dir":"UP", "num":8},
    {"dir":"DOWN", "num":5},
    {"dir":"DOWN", "num":1},
    {"dir":"FORWARD", "num":1},
    {"dir":"DOWN", "num":9},
    {"dir":"FORWARD", "num":5},
    {"dir":"DOWN", "num":1},
    {"dir":"FORWARD", "num":5},
    {"dir":"FORWARD", "num":6},
    {"dir":"UP", "num":2},
    {"dir":"FORWARD", "num":5},
    {"dir":"DOWN", "num":2},
    {"dir":"UP", "num":3},
    {"dir":"FORWARD", "num":8},
    {"dir":"FORWARD", "num":9},
    {"dir":"DOWN", "num":1},
    {"dir":"UP", "num":9},
    {"dir":"FORWARD", "num":6},
    {"dir":"DOWN", "num":9},
    {"dir":"DOWN", "num":2},
    {"dir":"FORWARD", "num":2},
    {"dir":"FORWARD", "num":4},
    {"dir":"FORWARD", "num":9},
    {"dir":"FORWARD", "num":3},
    {"dir":"DOWN", "num":6},
    {"dir":"DOWN", "num":9},
    {"dir":"FORWARD", "num":8},
    {"dir":"FORWARD", "num":3},
    {"dir":"DOWN", "num":9},
    {"dir":"UP", "num":6},
    {"dir":"FORWARD", "num":5},
    {"dir":"FORWARD", "num":8},
    {"dir":"DOWN", "num":7},
    {"dir":"FORWARD", "num":9},
    {"dir":"FORWARD", "num":5},
    {"dir":"FORWARD", "num":4},
    {"dir":"FORWARD", "num":3},
    {"dir":"DOWN", "num":9},
    {"dir":"FORWARD", "num":7},
    {"dir":"FORWARD", "num":7},
    {"dir":"FORWARD", "num":8},
    {"dir":"DOWN", "num":7},
    {"dir":"DOWN", "num":3},
    {"dir":"UP", "num":5},
    {"dir":"FORWARD", "num":2},
    {"dir":"UP", "num":6},
    {"dir":"DOWN", "num":6},
    {"dir":"DOWN", "num":5},
    {"dir":"UP", "num":2},
    {"dir":"DOWN", "num":1},
    {"dir":"FORWARD", "num":7},
    {"dir":"FORWARD", "num":3},
    {"dir":"DOWN", "num":2},
    {"dir":"UP", "num":4},
    {"dir":"FORWARD", "num":8},
    {"dir":"DOWN", "num":8},
    {"dir":"DOWN", "num":2},
    {"dir":"FORWARD", "num":8},
    {"dir":"UP", "num":8},
    {"dir":"FORWARD", "num":1},
    {"dir":"FORWARD", "num":1},
    {"dir":"DOWN", "num":1},
    {"dir":"FORWARD", "num":4},
    {"dir":"FORWARD", "num":7},
    {"dir":"FORWARD", "num":2},
    {"dir":"FORWARD", "num":6},
    {"dir":"DOWN", "num":9},
    {"dir":"DOWN", "num":2},
    {"dir":"UP", "num":7},
    {"dir":"UP", "num":5},
    {"dir":"FORWARD", "num":4},
    {"dir":"DOWN", "num":3},
    {"dir":"DOWN", "num":4},
    {"dir":"UP", "num":7},
    {"dir":"FORWARD", "num":7},
    {"dir":"UP", "num":6},
    {"dir":"FORWARD", "num":6},
    {"dir":"UP", "num":4},
    {"dir":"FORWARD", "num":5},
    {"dir":"FORWARD", "num":7},
    {"dir":"FORWARD", "num":3},
    {"dir":"DOWN", "num":2},
    {"dir":"FORWARD", "num":2},
    {"dir":"FORWARD", "num":2},
    {"dir":"UP", "num":1},
    {"dir":"DOWN", "num":8},
    {"dir":"UP", "num":7},
    {"dir":"DOWN", "num":4},
    {"dir":"DOWN", "num":9},
    {"dir":"FORWARD", "num":2},
    {"dir":"FORWARD", "num":9},
    {"dir":"DOWN", "num":7},
    {"dir":"DOWN", "num":4},
    {"dir":"DOWN", "num":8},
    {"dir":"FORWARD", "num":9},
    {"dir":"UP", "num":1},
    {"dir":"FORWARD", "num":6},
    {"dir":"FORWARD", "num":7},
    {"dir":"DOWN", "num":3},
    {"dir":"FORWARD", "num":9},
    {"dir":"FORWARD", "num":3},
    {"dir":"DOWN", "num":3},
    {"dir":"UP", "num":2},
    {"dir":"DOWN", "num":8},
    {"dir":"DOWN", "num":7},
    {"dir":"FORWARD", "num":1},
    {"dir":"DOWN", "num":9},
    {"dir":"UP", "num":3},
    {"dir":"FORWARD", "num":6},
    {"dir":"FORWARD", "num":1},
    {"dir":"FORWARD", "num":7},
    {"dir":"DOWN", "num":3},
    {"dir":"FORWARD", "num":2},
    {"dir":"FORWARD", "num":1},
    {"dir":"DOWN", "num":8},
    {"dir":"DOWN", "num":7},
    {"dir":"FORWARD", "num":1},
    {"dir":"FORWARD", "num":5},
    {"dir":"DOWN", "num":7},
    {"dir":"UP", "num":7},
    {"dir":"FORWARD", "num":4},
    {"dir":"DOWN", "num":3},
    {"dir":"FORWARD", "num":7},
    {"dir":"FORWARD", "num":2},
    {"dir":"UP", "num":2},
    {"dir":"FORWARD", "num":1},
    {"dir":"UP", "num":2},
    {"dir":"UP", "num":6},
    {"dir":"FORWARD", "num":3},
    {"dir":"FORWARD", "num":2},
    {"dir":"DOWN", "num":3},
    {"dir":"DOWN", "num":7},
    {"dir":"DOWN", "num":2},
    {"dir":"DOWN", "num":9},
    {"dir":"DOWN", "num":6},
    {"dir":"FORWARD", "num":8},
    {"dir":"DOWN", "num":4},
    {"dir":"FORWARD", "num":6},
    {"dir":"UP", "num":7},
    {"dir":"DOWN", "num":8},
    {"dir":"UP", "num":5},
    {"dir":"DOWN", "num":8},
    {"dir":"UP", "num":6},
    {"dir":"DOWN", "num":2},
    {"dir":"DOWN", "num":1},
    {"dir":"FORWARD", "num":9},
    {"dir":"FORWARD", "num":8},
    {"dir":"DOWN", "num":6},
    {"dir":"DOWN", "num":3},
    {"dir":"UP", "num":8},
    {"dir":"DOWN", "num":1},
    {"dir":"FORWARD", "num":2},
    {"dir":"DOWN", "num":6},
    {"dir":"FORWARD", "num":4},
    {"dir":"FORWARD", "num":9},
    {"dir":"FORWARD", "num":6},
    {"dir":"DOWN", "num":2},
    {"dir":"DOWN", "num":9},
    {"dir":"UP", "num":3},
    {"dir":"FORWARD", "num":8},
    {"dir":"FORWARD", "num":1},
    {"dir":"FORWARD", "num":6},
    {"dir":"FORWARD", "num":8},
    {"dir":"FORWARD", "num":4},
    {"dir":"FORWARD", "num":8},
    {"dir":"UP", "num":7},
    {"dir":"DOWN", "num":7},
    {"dir":"FORWARD", "num":7},
    {"dir":"DOWN", "num":6},
    {"dir":"FORWARD", "num":1},
    {"dir":"DOWN", "num":9},
    {"dir":"FORWARD", "num":5},
    {"dir":"DOWN", "num":3},
    {"dir":"DOWN", "num":6},
    {"dir":"DOWN", "num":6},
    {"dir":"DOWN", "num":9},
    {"dir":"FORWARD", "num":8},
    {"dir":"FORWARD", "num":5},
    {"dir":"DOWN", "num":4},
    {"dir":"FORWARD", "num":1},
    {"dir":"DOWN", "num":8},
    {"dir":"DOWN", "num":5},
    {"dir":"DOWN", "num":5},
    {"dir":"DOWN", "num":9},
    {"dir":"FORWARD", "num":1},
    {"dir":"FORWARD", "num":1},
    {"dir":"FORWARD", "num":9},
    {"dir":"FORWARD", "num":5},
    {"dir":"DOWN", "num":4},
    {"dir":"FORWARD", "num":3},
    {"dir":"DOWN", "num":2},
    {"dir":"FORWARD", "num":3},
    {"dir":"UP", "num":7},
    {"dir":"DOWN", "num":2},
    {"dir":"DOWN", "num":5},
    {"dir":"DOWN", "num":4},
    {"dir":"UP", "num":2},
    {"dir":"FORWARD", "num":7},
    {"dir":"DOWN", "num":3},
    {"dir":"DOWN", "num":4},
    {"dir":"DOWN", "num":8},
    {"dir":"DOWN", "num":2},
    {"dir":"FORWARD", "num":3},
    {"dir":"UP", "num":8},
    {"dir":"DOWN", "num":9},
    {"dir":"DOWN", "num":1},
    {"dir":"DOWN", "num":2},
    {"dir":"FORWARD", "num":3},
    {"dir":"DOWN", "num":1},
    {"dir":"DOWN", "num":7},
    {"dir":"DOWN", "num":9},
    {"dir":"FORWARD", "num":9},
    {"dir":"FORWARD", "num":7},
    {"dir":"DOWN", "num":2},
    {"dir":"DOWN", "num":5},
    {"dir":"UP", "num":8},
    {"dir":"FORWARD", "num":1},
    {"dir":"FORWARD", "num":2},
    {"dir":"FORWARD", "num":6},
    {"dir":"UP", "num":3},
    {"dir":"FORWARD", "num":3},
    {"dir":"DOWN", "num":6},
    {"dir":"FORWARD", "num":1},
    {"dir":"UP", "num":8},
    {"dir":"DOWN", "num":3},
    {"dir":"DOWN", "num":1},
    {"dir":"FORWARD", "num":6},
    {"dir":"UP", "num":7},
    {"dir":"FORWARD", "num":3},
    {"dir":"DOWN", "num":5},
    {"dir":"DOWN", "num":5},
    {"dir":"UP", "num":8},
    {"dir":"DOWN", "num":4},
    {"dir":"FORWARD", "num":1},
    {"dir":"UP", "num":5},
    {"dir":"DOWN", "num":4},
    {"dir":"FORWARD", "num":5},
    {"dir":"UP", "num":6},
    {"dir":"FORWARD", "num":2},
    {"dir":"UP", "num":4},
    {"dir":"FORWARD", "num":9},
    {"dir":"FORWARD", "num":1},
    {"dir":"FORWARD", "num":9},
    {"dir":"FORWARD", "num":5},
    {"dir":"FORWARD", "num":9},
    {"dir":"UP", "num":2},
    {"dir":"FORWARD", "num":8},
    {"dir":"FORWARD", "num":6},
    {"dir":"DOWN", "num":5},
    {"dir":"FORWARD", "num":9},
    {"dir":"FORWARD", "num":7},
    {"dir":"DOWN", "num":1},
    {"dir":"FORWARD", "num":7},
    {"dir":"FORWARD", "num":7},
    {"dir":"FORWARD", "num":3},
    {"dir":"UP", "num":9},
    {"dir":"DOWN", "num":2},
    {"dir":"UP", "num":7},
    {"dir":"DOWN", "num":1},
    {"dir":"UP", "num":7},
    {"dir":"FORWARD", "num":6},
    {"dir":"FORWARD", "num":2},
    {"dir":"FORWARD", "num":5},
    {"dir":"FORWARD", "num":6},
    {"dir":"DOWN", "num":4},
    {"dir":"DOWN", "num":8},
    {"dir":"FORWARD", "num":6},
    {"dir":"DOWN", "num":1},
    {"dir":"FORWARD", "num":7},
    {"dir":"UP", "num":8},
    {"dir":"FORWARD", "num":7},
    {"dir":"UP", "num":5},
    {"dir":"DOWN", "num":2},
    {"dir":"FORWARD", "num":9},
    {"dir":"FORWARD", "num":7},
    {"dir":"FORWARD", "num":8},
    {"dir":"UP", "num":2},
    {"dir":"DOWN", "num":9},
    {"dir":"FORWARD", "num":5},
    {"dir":"FORWARD", "num":8},
    {"dir":"FORWARD", "num":5},
    {"dir":"FORWARD", "num":7},
    {"dir":"DOWN", "num":8},
    {"dir":"DOWN", "num":8},
    {"dir":"UP", "num":4},
    {"dir":"DOWN", "num":8},
    {"dir":"UP", "num":2},
    {"dir":"UP", "num":6},
    {"dir":"DOWN", "num":2},
    {"dir":"FORWARD", "num":2},
    {"dir":"FORWARD", "num":9},
    {"dir":"FORWARD", "num":8},
    {"dir":"FORWARD", "num":3},
    {"dir":"UP", "num":3},
    {"dir":"UP", "num":1},
    {"dir":"DOWN", "num":3},
    {"dir":"FORWARD", "num":7},
    {"dir":"DOWN", "num":2},
    {"dir":"UP", "num":2},
    {"dir":"FORWARD", "num":5},
    {"dir":"DOWN", "num":3},
    {"dir":"FORWARD", "num":2},
    {"dir":"FORWARD", "num":4},
    {"dir":"UP", "num":2},
    {"dir":"DOWN", "num":1},
    {"dir":"DOWN", "num":5},
    {"dir":"UP", "num":6},
    {"dir":"DOWN", "num":7},
    {"dir":"UP", "num":7},
    {"dir":"UP", "num":6},
    {"dir":"FORWARD", "num":5},
    {"dir":"DOWN", "num":7},
    {"dir":"FORWARD", "num":9},
    {"dir":"FORWARD", "num":2},
    {"dir":"UP", "num":8},
    {"dir":"DOWN", "num":1},
    {"dir":"DOWN", "num":6},
    {"dir":"FORWARD", "num":9},
    {"dir":"DOWN", "num":8},
    {"dir":"UP", "num":7},
    {"dir":"DOWN", "num":6},
    {"dir":"FORWARD", "num":9},
    {"dir":"FORWARD", "num":2},
    {"dir":"DOWN", "num":1},
    {"dir":"UP", "num":4},
    {"dir":"UP", "num":7},
    {"dir":"DOWN", "num":7},
    {"dir":"UP", "num":6},
    {"dir":"FORWARD", "num":4},
    {"dir":"UP", "num":1},
    {"dir":"FORWARD", "num":5},
    {"dir":"FORWARD", "num":9},
    {"dir":"DOWN", "num":3},
    {"dir":"UP", "num":2},
    {"dir":"DOWN", "num":3},
    {"dir":"UP", "num":7},
    {"dir":"DOWN", "num":8},
    {"dir":"FORWARD", "num":1},
    {"dir":"DOWN", "num":9},
    {"dir":"DOWN", "num":1},
    {"dir":"DOWN", "num":9},
    {"dir":"FORWARD", "num":8},
    {"dir":"DOWN", "num":4},
    {"dir":"DOWN", "num":7},
    {"dir":"DOWN", "num":7},
    {"dir":"FORWARD", "num":4},
    {"dir":"UP", "num":5},
    {"dir":"FORWARD", "num":1},
    {"dir":"FORWARD", "num":7},
    {"dir":"DOWN", "num":9},
    {"dir":"FORWARD", "num":6},
    {"dir":"UP", "num":8},
    {"dir":"DOWN", "num":5},
    {"dir":"DOWN", "num":8},
    {"dir":"UP", "num":9},
    {"dir":"FORWARD", "num":6},
    {"dir":"DOWN", "num":1},
    {"dir":"FORWARD", "num":8},
    {"dir":"UP", "num":8},
    {"dir":"UP", "num":1},
    {"dir":"UP", "num":7},
    {"dir":"DOWN", "num":9},
    {"dir":"FORWARD", "num":8},
    {"dir":"DOWN", "num":8},
    {"dir":"FORWARD", "num":4},
    {"dir":"DOWN", "num":1},
    {"dir":"DOWN", "num":5},
    {"dir":"FORWARD", "num":9},
    {"dir":"DOWN", "num":7},
    {"dir":"FORWARD", "num":3},
    {"dir":"FORWARD", "num":1},
    {"dir":"FORWARD", "num":2},
    {"dir":"DOWN", "num":9},
    {"dir":"UP", "num":2},
    {"dir":"UP", "num":1},
    {"dir":"FORWARD", "num":6},
    {"dir":"UP", "num":1},
    {"dir":"DOWN", "num":3},
    {"dir":"FORWARD", "num":3},
    {"dir":"DOWN", "num":7},
    {"dir":"DOWN", "num":4},
    {"dir":"FORWARD", "num":2},
    {"dir":"FORWARD", "num":8},
    {"dir":"FORWARD", "num":7},
    {"dir":"DOWN", "num":6},
    {"dir":"UP", "num":1},
    {"dir":"FORWARD", "num":8},
    {"dir":"DOWN", "num":8},
    {"dir":"FORWARD", "num":5},
    {"dir":"DOWN", "num":4},
    {"dir":"FORWARD", "num":2},
    {"dir":"UP", "num":8},
    {"dir":"DOWN", "num":5},
    {"dir":"DOWN", "num":9},
    {"dir":"FORWARD", "num":1},
    {"dir":"DOWN", "num":8},
    {"dir":"DOWN", "num":5},
    {"dir":"FORWARD", "num":6},
    {"dir":"DOWN", "num":1},
    {"dir":"DOWN", "num":6},
    {"dir":"UP", "num":5},
    {"dir":"UP", "num":8},
    {"dir":"FORWARD", "num":7},
    {"dir":"FORWARD", "num":4},
    {"dir":"DOWN", "num":5},
    {"dir":"DOWN", "num":1},
    {"dir":"FORWARD", "num":4},
    {"dir":"UP", "num":9},
    {"dir":"FORWARD", "num":9},
    {"dir":"UP", "num":7},
    {"dir":"DOWN", "num":4},
    {"dir":"FORWARD", "num":8},
    {"dir":"FORWARD", "num":5},
    {"dir":"UP", "num":8},
    {"dir":"DOWN", "num":9},
    {"dir":"UP", "num":8},
    {"dir":"FORWARD", "num":7},
    {"dir":"DOWN", "num":8},
    {"dir":"FORWARD", "num":4},
    {"dir":"DOWN", "num":8},
    {"dir":"UP", "num":7},
    {"dir":"FORWARD", "num":4},
    {"dir":"DOWN", "num":9},
    {"dir":"FORWARD", "num":5},
    {"dir":"UP", "num":7},
    {"dir":"FORWARD", "num":5},
    {"dir":"FORWARD", "num":9},
    {"dir":"DOWN", "num":6},
    {"dir":"UP", "num":6},
    {"dir":"FORWARD", "num":2},
    {"dir":"DOWN", "num":4},
    {"dir":"DOWN", "num":8},
    {"dir":"FORWARD", "num":9},
    {"dir":"FORWARD", "num":4},
    {"dir":"FORWARD", "num":6},
    {"dir":"UP", "num":6},
    {"dir":"DOWN", "num":6},
    {"dir":"FORWARD", "num":6},
    {"dir":"FORWARD", "num":7},
    {"dir":"FORWARD", "num":5},
    {"dir":"FORWARD", "num":3},
    {"dir":"FORWARD", "num":7},
    {"dir":"UP", "num":3},
    {"dir":"DOWN", "num":6},
    {"dir":"FORWARD", "num":2},
    {"dir":"DOWN", "num":9},
    {"dir":"UP", "num":8},
    {"dir":"DOWN", "num":2},
    {"dir":"FORWARD", "num":7},
    {"dir":"DOWN", "num":9},
    {"dir":"DOWN", "num":3},
    {"dir":"UP", "num":9},
    {"dir":"UP", "num":6},
    {"dir":"DOWN", "num":6},
    {"dir":"FORWARD", "num":8},
    {"dir":"UP", "num":2},
    {"dir":"UP", "num":1},
    {"dir":"DOWN", "num":5},
    {"dir":"FORWARD", "num":4},
    {"dir":"DOWN", "num":2},
    {"dir":"UP", "num":2},
    {"dir":"FORWARD", "num":2},
    {"dir":"FORWARD", "num":3},
    {"dir":"UP", "num":5},
    {"dir":"DOWN", "num":6},
    {"dir":"FORWARD", "num":9},
    {"dir":"UP", "num":3},
    {"dir":"FORWARD", "num":6},
    {"dir":"FORWARD", "num":4},
    {"dir":"DOWN", "num":3},
    {"dir":"UP", "num":9},
    {"dir":"FORWARD", "num":1},
    {"dir":"UP", "num":6},
    {"dir":"DOWN", "num":9},
    {"dir":"DOWN", "num":7},
    {"dir":"FORWARD", "num":5},
    {"dir":"DOWN", "num":2},
    {"dir":"DOWN", "num":6},
    {"dir":"FORWARD", "num":5},
    {"dir":"FORWARD", "num":1},
    {"dir":"FORWARD", "num":5},
    {"dir":"FORWARD", "num":8},
    {"dir":"UP", "num":2},
    {"dir":"FORWARD", "num":9},
    {"dir":"DOWN", "num":8},
    {"dir":"FORWARD", "num":2},
    {"dir":"DOWN", "num":6},
    {"dir":"DOWN", "num":1},
    {"dir":"DOWN", "num":9},
    {"dir":"FORWARD", "num":6}]"#.to_string()
}
