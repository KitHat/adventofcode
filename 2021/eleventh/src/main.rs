fn main() {
    println!("Hello, world!");
    let test = read_file("test.txt");
    let source = read_file("source.txt");
    println!("test 2 {}", solve_2(test));
    println!("source 2 {}", solve_2(source));

}

fn solve_1(mut input: [[i64; 12]; 12]) -> i32 {
    // println!("{:?}", input);
    let mut res = 0;
    for s in 0..100 {
        //increase
        for i in 1..11 {
            for j in 1..11 {
                input[i][j] += 1;
            }
        }
        
        let mut flashed = true;
        while flashed {
            //calculate
            flashed = false;
            for i in 1..11 {
                for j in 1..11 {
                    if input[i][j] > 9 {
                        input[i][j] = 0;
                        res += 1;
                        flashed = true;
                        if input[i-1][j-1] != 0 {
                            input[i-1][j-1] += 1
                        }
                        if input[i-1][j] != 0 {
                            input[i-1][j] += 1
                        }
                        if input[i-1][j+1] != 0 {
                            input[i-1][j+1] += 1
                        }
                        if input[i][j-1] != 0 {
                            input[i][j-1] += 1
                        }
                        if input[i][j+1] != 0 {
                            input[i][j+1] += 1
                        }
                        if input[i+1][j-1] != 0 {
                            input[i+1][j-1] += 1
                        }
                        if input[i+1][j] != 0 {
                            input[i+1][j] += 1
                        }
                        if input[i+1][j+1] != 0 {
                            input[i+1][j+1] += 1
                        }
                    }
                }
            }
        }
    }
    res
}

fn solve_2(mut input: [[i64; 12]; 12]) -> i32 {
    // println!("{:?}", input);
    let mut res = 0;
    let mut step = 0;
    loop {
        step += 1;
        //increase
        for i in 1..11 {
            for j in 1..11 {
                input[i][j] += 1;
            }
        }
        
        let mut flashed = true;
        while flashed {
            //calculate
            flashed = false;
            for i in 1..11 {
                for j in 1..11 {
                    if input[i][j] > 9 {
                        input[i][j] = 0;
                        res += 1;
                        flashed = true;
                        if input[i-1][j-1] != 0 {
                            input[i-1][j-1] += 1
                        }
                        if input[i-1][j] != 0 {
                            input[i-1][j] += 1
                        }
                        if input[i-1][j+1] != 0 {
                            input[i-1][j+1] += 1
                        }
                        if input[i][j-1] != 0 {
                            input[i][j-1] += 1
                        }
                        if input[i][j+1] != 0 {
                            input[i][j+1] += 1
                        }
                        if input[i+1][j-1] != 0 {
                            input[i+1][j-1] += 1
                        }
                        if input[i+1][j] != 0 {
                            input[i+1][j] += 1
                        }
                        if input[i+1][j+1] != 0 {
                            input[i+1][j+1] += 1
                        }
                    }
                }
            }
        }

        if all_zeros(&input) {
            return step;
        }
    }
    -1
}

fn all_zeros(input: &[[i64; 12]; 12]) -> bool {
    
    for i in 1..11 {
        for j in 1..11 {
            if input[i][j] != 0 {
                return false;
            }
        }
    }
    true
}


fn read_file(file: &str) -> [[i64; 12]; 12] {
    let mut res = [[std::i64::MIN; 12]; 12];
    let contents = std::fs::read_to_string(file).unwrap();
    for (i, s) in contents.split('\n').enumerate() {
        for (j, c) in s.trim().chars().enumerate() {
            res[i+1][j+1] = c.to_string().parse::<i64>().unwrap();
        }
    }
    res
}
