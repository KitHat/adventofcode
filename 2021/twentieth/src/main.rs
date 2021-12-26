use std::collections::HashMap;
fn main() {
    println!("Hello, world!");
    let (mut start, enhancement) = read_data("test.txt");
    
    for i in 0..50 {
        start = enhance(start, &enhancement, i%2);
    }
    
    // let iter_2 = enhance(iter_1, &enhancement, 100);
    let res = start.iter().filter(|((_, _),v)|  **v == 1).count();
    println!("answer {}", res);
}

fn print_image(image: &HashMap<Coord, usize>) {
    
    let min_x = image.iter().map(|((x, _), _)| x).min().unwrap();
    let max_x = image.iter().map(|((x, _), _)| x).max().unwrap();
    let min_y = image.iter().map(|((_, y), _)| y).min().unwrap();
    let max_y = image.iter().map(|((y, _), _)| y).max().unwrap();
    for i in min_x-1..max_x+1 {
        for j in min_y-1..max_y+1 {
            match image.get(&(i,j)).unwrap_or(&0) {
                1 => print!("#"),
                0 => print!("."),
                _ => ()
            }
        }
        println!();
    }
}

fn enhance(image: HashMap<Coord, usize>, enhancement: &Vec<usize>, unknown: usize) -> HashMap<Coord, usize> {
    let mut res = HashMap::<Coord, usize>::new();
    let min_x = image.iter().map(|((x, _), _)| x).min().unwrap();
    let max_x = image.iter().map(|((x, _), _)| x).max().unwrap();
    let min_y = image.iter().map(|((_, y), _)| y).min().unwrap();
    let max_y = image.iter().map(|((_, y), _)| y).max().unwrap();
    for i in min_x-1..max_x+2 {
        for j in min_y-1..max_y+2 {
            let index = neighbours((i, j)).iter().map(|c| image.get(&c).unwrap_or(&unknown)).fold(0, |acc, next| (acc << 1) + next);
            res.insert((i, j), enhancement[index]);
        }
    }
    res
}

fn neighbours(c: Coord) -> Vec<Coord> {
    vec![
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 0),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1)
    ].iter().map(|a| (c.0+a.0, c.1+a.1)).collect()
}

type Coord = (isize, isize);

fn read_data(input: &str) -> (HashMap<Coord, usize>, Vec<usize>) {
    let mut enhancement = vec![];
    let mut input_image = HashMap::<Coord, usize>::new();
    let contents = std::fs::read_to_string(input).unwrap();
    let csplit = contents.split("\r\n\r\n").collect::<Vec<&str>>();
    for c in csplit[0].chars() {
        if c == '.' {
            enhancement.push(0);
        } else if c == '#' {
            enhancement.push(1);
        }
    }
    for (i, s) in csplit[1].split("\r\n").enumerate() {
        for (j, c) in s.chars().enumerate() {
            if c == '.' {
                input_image.insert((i as isize,j as isize), 0);
            } else if c == '#' {
                input_image.insert((i as isize,j as isize), 1);
            }
        }
    }

    (input_image, enhancement)
}
