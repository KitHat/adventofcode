fn main() {
    println!("Hello, world!");
    eval_values();
}
#[macro_use]
extern crate lazy_static;

use cached::proc_macro::cached;

lazy_static! {
    static ref MONAD: Vec<Vec<Instruction>> = read_data("input.txt");
}

fn eval_values() {
    println!("{}", do_eval_values(0, 0));
}

#[cached]
fn do_eval_values(z: i64, n: usize) -> i64 {
    for i in 1..=9 {
        let z_next = eval_block(i, z, n);
        if n+1 == 14 {
            //time to check!
            if z_next == 0 {
                return i;
            }
        } else {
            let val = do_eval_values(z_next, n+1); 
            if val != -1 {
                let mut res = i;
                for _ in 0..14-n {
                    res *=10
                }
                return res + val;
            }
        }
    }
    -1
}

fn eval_block(w: i64, z: i64, n: usize) -> i64 {
    let mut x = 0;
    let mut y = 0;
    let mut next_z = z; 
    let mut next_w = w;
    for instruction in &MONAD[n] {
        let val1 = match instruction.val1.clone() {
            Registry::W => next_w,
            Registry::X => x,
            Registry::Y => y,
            Registry::Z => next_z,
            Registry::Val(v) => v,
        };
        let val2 = match instruction.val2.clone() {
            Registry::W => next_w,
            Registry::X => x,
            Registry::Y => y,
            Registry::Z => z,
            Registry::Val(v) => v,
        };
        let res = match instruction.op.clone() {
            Op::Add => val1+val2,
            Op::Div => val1/val2,
            Op::Eql => if val1 == val2 {1} else {0},
            Op::Mod => val1%val2,
            Op::Mul => val1*val2
        };        
        match instruction.val1.clone() {
            Registry::W => next_w = res,
            Registry::X => x = res,
            Registry::Y => y = res,
            Registry::Z => next_z = res,
            Registry::Val(_) => unreachable!(),
        };
    }
    next_z
}

#[derive(Clone)]
enum Registry {
    X,
    Y,
    Z,
    W,
    Val(i64)
}

#[derive(Clone)]
enum Op {
    Eql,
    Mod,
    Div,
    Add,
    Mul
}

struct Instruction {
    val1: Registry,
    val2: Registry,
    op: Op
}

fn read_data(file: &str) -> Vec<Vec<Instruction>> {
    let mut res = vec![];
    let mut next: Vec<Instruction> = vec![];
    let data = std::fs::read_to_string(file).unwrap();
    for s in data.split("\r\n") {
        let split = s.split(" ").collect::<Vec<&str>>();
        if split.len() == 2 {
            //it is input
            if !(next.len() == 0) {
                res.push(next);
                next = vec![];
            }
        } else {
            let op = match split[0] {
                "mul" => Op::Mul,
                "add" => Op::Add,
                "mod" => Op::Mod,
                "eql" => Op::Eql,
                "div" => Op::Div,
                s     => {
                    println!("{}", s);
                    unreachable!()
                }
            };
            let val1 = match split[1] {
                "x" => Registry::X,
                "y" => Registry::Y,
                "z" => Registry::Z,
                "w" => Registry::W,
                s   => Registry::Val(s.parse::<i64>().unwrap())
            };
            let val2 = match split[2] {
                "x" => Registry::X,
                "y" => Registry::Y,
                "z" => Registry::Z,
                "w" => Registry::W,
                s   => Registry::Val(s.parse::<i64>().unwrap())
            };
            next.push(Instruction{op, val1, val2});
        }
    }
    res.push(next);
    res
}