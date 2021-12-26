use std::collections::HashMap;

fn main() {
    println!("Hello, world!");
    let test = read_file("test.txt");
    let source = read_file("source.txt");
    println!("1 test {}", solve_1(&test));
    println!("1 source {}", solve_1(&source));
    println!("2 test {}", solve_1(&expand_map(&test)));
    println!("2 source {}", solve_1(&expand_map(&source)));
}

fn solve_1(input: &Vec<Vec<i32>>) -> i32 {
    let len = input.len(); 
    // let mut calc = vec![vec![300000; len+2]; len+2];
    let mut walks: HashMap<(i32, i32), i32> = HashMap::new();
    for i in 0..input.len() {
        for j in 0..input.len() {
            let key = (i as i32,j as i32);
            *walks.entry(key).or_insert(i32::MAX) = i32::MAX;
        }
    }
    *walks.entry((0,0)).or_insert(0) = 0;
    let mut q = std::collections::BinaryHeap::new();
    q.push(std::cmp::Reverse(((0,0), 0)));
    while !q.is_empty() {
        let std::cmp::Reverse(((x,y), len)) = q.pop().unwrap();
        // println!("{} {} len {}", x, y, len);
        if x as usize == input.len() - 1 && y as usize == input.len() - 1 {
            return len;
        }
        if *walks.get(&(x,y)).unwrap() < len {continue;}
        for i in [(0,1), (0, -1), (1, 0), (-1, 0)] {
            let next_x = x+i.0;
            let next_y = y+i.1;
            if let Some(next) = walks.get_mut(&(next_x, next_y)) {
                let next_val = len + input[next_x as usize][next_y as usize];
                if *next > next_val {
                    // println!("relax {} {} {}", next_x, next_y, next_val);
                    *next = next_val;
                    q.push(std::cmp::Reverse(((next_x, next_y), next_val)));
                }
            }
        }
    };
    0
}

fn expand_map(input: &Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let len = input.len();
    let mut res: Vec<Vec<i32>> = vec![vec![0; len*5]; len*5];
    for i in 0usize..5 {
        for j in 0usize..5 {
            for k in 0..len {
                for l in 0..len {
                    let mut nval = input[k][l] + i as i32 + j as i32;
                    while nval > 9 {nval -= 9}
                    res[len*i+k][len*j+l] = nval;
                }
            }
        }
    }
    res
}

fn read_file(file: &str) -> Vec<Vec<i32>> {
    let contents = std::fs::read_to_string(file).unwrap();
    let mut res: Vec<Vec<i32>> = vec![];
    for s in contents.split("\r\n") {
        let mut next: Vec<i32> = vec![];
        for c in s.chars() {
            let i = c.to_digit(10).unwrap();
            next.push(i as i32);
        }
        res.push(next);
    }
    res
}
