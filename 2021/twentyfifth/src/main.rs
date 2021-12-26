use std::collections::HashSet;

fn main() {
    println!("Hello, world!");
    let (mut east, mut south, len, width) = read_data("input.txt");
    println!("{} {}", len, width);
    let mut i = 0;
    let mut steps_east: Vec<(Cucumber, Cucumber)> = vec![];
    let mut steps_count = 0;
    let mut steps_south: Vec<(Cucumber, Cucumber)> = vec![];
    while i == 0 || steps_count != 0 {
        steps_count = 0;
        steps_east.clear();
        steps_south.clear();
        for cucumber in east.iter() {
            let next_cucumber = next_pos(&cucumber, width, true);
            if !east.contains(&next_cucumber) && !south.contains(&next_cucumber) {
                steps_east.push((cucumber.clone(), next_cucumber));
            }
        }
        steps_count += steps_east.len();
        for (to_remove, to_add) in &steps_east {
            east.remove(to_remove);
            east.insert(*to_add);
        }
        for cucumber in south.iter() {
            let next_cucumber = next_pos(&cucumber, len, false);
            if !east.contains(&next_cucumber) && !south.contains(&next_cucumber) {
                steps_south.push((cucumber.clone(), next_cucumber));
            }
        }
        steps_count += steps_south.len();
        for (to_remove, to_add) in &steps_south {
            south.remove(to_remove);
            south.insert(*to_add);
        }
        i+=1;
        // pretty_print(len, width, &east, &south);
    }
    println!("{}", i);
}

fn pretty_print(len: usize, width: usize, east: &HashSet<Cucumber>, south: &HashSet<Cucumber>) {
    for i in 0..len {
        for j in 0..width {
            if east.contains(&(i,j)) {
                print!(">");
            } else if south.contains(&(i,j)) {
                print!("v");
            } else {
                print!(".");
            }
        }
        println!("");
    }
}

fn next_pos(cur: &Cucumber, len: usize, east: bool) -> Cucumber {
    if east {
        (cur.0, if cur.1 == len-1 {0} else {cur.1+1})
    } else {
        (if cur.0 == len-1 {0} else {cur.0+1}, cur.1)
    }
}


type Cucumber = (usize, usize);

fn read_data(file: &str) -> (HashSet<Cucumber>, HashSet<Cucumber>, usize, usize) {
    let data = std::fs::read_to_string(file).unwrap();
    let mut east = HashSet::new();
    let mut south = HashSet::new();
    let len = data.split("\r\n").count();
    let mut width = 0;
    for (i, s) in data.split("\r\n").enumerate() {
        width = s.len();
        for (j, c) in s.chars().enumerate() {
            if c == '>' {
                east.insert((i, j));
            } else if c == 'v' {
                south.insert((i,j));
            }
        }
    }
    (east, south, len, width)
}