fn main() {
    let test = read_contents("test.txt");
    let op = parse_operator(&test, &mut 0);
    println!("{:?}", calc_vers(&op));
}

// #[derive(PartialEq)]
// enum State {
//     Start,
//     Version(u64, u64), // res, accumulated bytes
//     Type(u64, u64), // res, accumulated bytes
//     LenType,
//     LenBytes(u64, u64), // res, len
//     LenNums(u64, u64), // res, len
//     Val(u64, u64, bool), // res, acc, last
//     Finish,
//     Next
// }

fn calc_vers(op: &Operator) -> u64 {
    match op.typ.clone() {
        Type::Operand(0) => {
            op.ops.iter().map(|o| calc_vers(o)).sum()
        },
        Type::Operand(1) => {
            op.ops.iter().map(|o| calc_vers(o)).fold(1, |acc, next| acc*next)
        },
        Type::Operand(2) => op.ops.iter().map(|o| calc_vers(o)).min().unwrap(),
        Type::Operand(3) => op.ops.iter().map(|o| calc_vers(o)).max().unwrap(),
        Type::Operand(5) => if calc_vers(&op.ops[0]) > calc_vers(&op.ops[1]) {1} else {0},
        Type::Operand(6) => if calc_vers(&op.ops[0]) < calc_vers(&op.ops[1]) {1} else {0},
        Type::Operand(7) => if calc_vers(&op.ops[0]) == calc_vers(&op.ops[1]) {1} else {0},
        Type::Operand(_) => 0,
        Type::Literal => op.val
    }
}

#[derive(Clone, PartialEq, Debug)]
enum Type {
    Operand(u64), //type val
    Literal
}

#[derive(PartialEq)]
enum LenType {
    Bytes,
    Nums,
    None
}

#[derive(Debug)]
struct Operator {
    typ: Type,
    ver: u64,
    ops: Vec<Operator>,
    val: u64
}

// fn parse_input(input: &Vec<usize>) -> Operator {
//     let mut state = State::Start;
//     let cur = Operator {
//         typ: Type::Operand(0),
//         ver: 0,
//         ops: vec![],
//         val: 0
//     };
//     let mut i = -1i64;
//     let mut stack: Vec<Operator> = vec![];
//     let mut len_type = LenType::None;
//     let mut len = 0;
//     let mut len_stack = vec![(LenType::None, 0)];
//     while state != State::Finish {
//         let bt = input[i as usize];
//         state = match state {
//             State::Start   => {
//                 let lens = len_stack.pop().unwrap();
//                 len_type = lens.0;
//                 len = lens.1;
//                 State::Version(0, 0)
//             },
//             //Parsing metadata
//             State::Version(acc, len) => {
//                 let new_acc = (acc << 1) + bt as u64;
//                 if len == 2 {
//                     cur.ver = new_acc;
//                     State::Type(0, 0)
//                 } else {
//                     State::Version(new_acc, len+1)
//                 }
//             },
//             State::Type(acc, len) => {
//                 let new_acc = (acc << 1) + bt as u64;
//                 if len == 2 {
//                     let typ = if new_acc == 4 {
//                         Type::Literal
//                     } else {
//                         Type::Operand(new_acc)
//                     };
//                     cur.typ = typ.clone();
//                     match typ {
//                         Type::Literal => State::Val(0, 0, false),
//                         Type::Operand(_) => State::LenType
//                     }
//                 } else {
//                     State::Type(new_acc, len+1)
//                 }
//             },
//             //Parsing literal
//             State::Val(res, cnt, last) => {
//                 if cnt == 0 {
//                     State::Val(res, cnt+1, bt == 0)
//                 } else {
//                     if cnt == 4 {
//                         cur.val <<= 4;
//                         let next_val = (res << 1) + bt as u64;
//                         cur.val += next_val;
//                         if last {
//                             State::Next
//                         } else {
//                             State::Val(0, 0, false)
//                         }
//                     } else {
//                         let next_res = (res << 1) + bt as u64;
//                         State::Val(next_res, cnt+1, last)
//                     }
//                 }
//             },
//             //Parsing operator
//             State::LenType => {
//                 if bt == 0 {
//                     State::LenBytes(0, 0)
//                 } else {
//                     State::LenNums(0, 0)
//                 }
//             },
//             State::LenBytes(acc, cnt) => {
//                 let next_val = (acc << 1) + bt as u64;
//                 if cnt == 14 {
//                     len_stack.push((LenType::Bytes, next_val));
//                     stack.push(cur);
//                     cur = Operator {
//                         typ: Type::Operand(0),
//                         ver: 0,
//                         ops: vec![],
//                         val: 0
//                     };
//                     stack.last_mut().unwrap().ops.push(cur);
//                     State::Start
//                 } else {
//                     State::LenBytes(next_val, cnt+1)
//                 }
//             },
//             State::LenNums(acc, cnt) => {
//                 let next_val = (acc << 1) + bt as u64;
//                 if cnt == 10 {
//                     len_stack.push((LenType::Nums, next_val));
//                     stack.push(cur);
//                     cur = Operator {
//                         typ: Type::Operand(0),
//                         ver: 0,
//                         ops: vec![],
//                         val: 0
//                     };
//                     stack.last_mut().unwrap().ops.push(cur);
//                     State::Start
//                 } else {
//                     State::LenNums(next_val, cnt+1)
//                 }
//             },
//             State::Next => {
//                 match len_stack.pop() {
//                     Some()
//                 }
//             },
//             // terminal
//             State::Finish => State::Finish
//             //
//         };
//         i+=1;
//     }
//     cur
// }

fn parse_operator(input: &Vec<usize>, index: &mut usize) -> Operator {
    // parse ver
    let mut val = 0;
    for i in 0..3 {
        val <<= 1;
        val += input[*index];
        *index += 1;
    }
    let ver = val;
    //parse typ
    val = 0;
    for i in 0..3 {
        val <<= 1;
        val += input[*index];
        *index += 1;
    }
    let typ = if val == 4 {Type::Literal} else {Type::Operand(val as u64)};
    let mut ops = vec![];
    let mut val_fin = 0;
    if typ == Type::Literal {
        while input[*index] != 0 {
            *index += 1;
            for i in 0..4 {
                val_fin <<= 1;
                val_fin += input[*index];
                *index += 1;
            }
        }
        *index += 1;
        for i in 0..4 {
            val_fin <<= 1;
            val_fin += input[*index];
            *index += 1;
        }
    } else {
        let len_type = if input[*index] == 0 {LenType::Bytes} else {LenType::Nums};
        *index+=1;
        if len_type == LenType::Bytes {
            let mut byte_len = 0;
            for i in 0..15 {
                byte_len <<= 1;
                byte_len += input[*index];
                *index += 1;
            }
            let index_start = *index;
            while *index < index_start + byte_len {
                let op = parse_operator(input, index);
                ops.push(op)
            }
        } else {
            let mut num_len = 0;
            for i in 0..11 {
                num_len <<= 1;
                num_len += input[*index];
                *index += 1;
            }
            for i in 0..num_len {
                let op = parse_operator(input, index);
                ops.push(op)
            }
        }
    }
    Operator {
        typ,
        ver: ver as u64,
        ops, 
        val: val_fin as u64
    }
}

fn read_contents(file: &str) -> Vec<usize> {
    let contents = std::fs::read_to_string(file).unwrap();
    contents.chars().map(|c| get_bytes(c)).flatten().collect()
}

fn get_bytes(c: char) -> Vec<usize> {
    match c {
        '0' => vec![0, 0, 0, 0],
        '1' => vec![0, 0, 0, 1],
        '2' => vec![0, 0, 1, 0],
        '3' => vec![0, 0, 1, 1],
        '4' => vec![0, 1, 0, 0],
        '5' => vec![0, 1, 0, 1],
        '6' => vec![0, 1, 1, 0],
        '7' => vec![0, 1, 1, 1], 
        '8' => vec![1, 0, 0, 0], 
        '9' => vec![1, 0, 0, 1],
        'A' => vec![1, 0, 1, 0],
        'B' => vec![1, 0, 1, 1],
        'C' => vec![1, 1, 0, 0],
        'D' => vec![1, 1, 0, 1],
        'E' => vec![1, 1, 1, 0],
        'F' => vec![1, 1, 1, 1],
        _ => vec![]
    }
}