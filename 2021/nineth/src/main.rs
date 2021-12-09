use std::collections::HashSet;
fn main() {
    let data =  input("source");
    let lows = solve_1(&data);
    println!("{:?}", solve_2(&data, lows));
}

fn solve_1(input: &Vec<Vec<u32>>) -> Vec<(usize, usize)> {
    let mut res: Vec<(usize, usize)> = vec![];
    for i in 1..input.len()-1 {
        for j in 1..input[i].len()-1{
            if input[i][j] < input[i-1][j] && input[i][j] < input[i][j-1] && input[i][j] < input[i+1][j] && input[i][j] < input[i][j+1] {
                res.push((i,j))
            }   
        }
    }
    res
}

fn solve_2(input: &Vec<Vec<u32>>, lows: Vec<(usize, usize)>) -> u32 {
    let mut basins: Vec<u32> = lows.into_iter().map(|(x,y)| {
        let mut basin: HashSet<(usize, usize)> = HashSet::new();
        let mut to_explore: Vec<(usize, usize)> = vec![(x,y)];
        while let Some((next_x, next_y)) = to_explore.pop() {
            basin.insert((next_x, next_y));
            let cur_height = input[next_x][next_y];
            let down_x = next_x-1;
            let up_x = next_x+1;
            let down_y = next_y-1;
            let up_y = next_y+1;
            if input[next_x][up_y] > cur_height && input[next_x][up_y] != 9 && !basin.contains(&(next_x, up_y)) {
                to_explore.push((next_x, up_y))
            }
            if input[next_x][down_y] > cur_height && input[next_x][down_y] != 9 && !basin.contains(&(next_x, down_y)) {
                to_explore.push((next_x, down_y))
            }
            if input[up_x][next_y] > cur_height && input[up_x][next_y] != 9 && !basin.contains(&(up_x, next_y)) {
                to_explore.push((up_x, next_y))
            }
            
            if input[down_x][next_y] > cur_height && input[down_x][next_y] != 9 && !basin.contains(&(down_x, next_y)) {
                to_explore.push((down_x, next_y))
            }
        }
        basin.len() as u32
    }).collect();
    basins.sort();
    basins.reverse();
    basins[0]*basins[1]*basins[2]
}

fn input(filename: &str) -> Vec<Vec<u32>> {
    let contents: String = std::fs::read_to_string(filename).unwrap();
    let height = contents.chars().filter(|c| *c == '\n').count();
    let mut res: Vec<Vec<u32>> = vec![vec![9; height+3]; height+3];
    const RADIX: u32 = 10;
    for (i, s) in contents.split("\n").enumerate() {
        for (j, c) in s.trim().chars().enumerate() {
            res[i+1][j+1] = c.to_digit(RADIX).unwrap();
        }
    }

    res
}
