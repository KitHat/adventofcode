use std::str::FromStr;
fn main() {
    println!("Hello, world!");
    let (test_coords, test_folds) = read_file("test.txt");
    let (source_coords, source_folds) = read_file("source.txt");
    let solution = solve(source_coords, source_folds);
    for i in 0..6 {
        for j in 0..40 {
            if solution.contains(&(j,i)) {
                print!("#")
            } else {
                print!(" ");
            }
        }
        println!("");
    }
}

fn solve(coords: Vec<(u32, u32)>, folds: Vec<(bool, u32)>) -> Vec<(u32, u32)> {
    let mut res = coords;
    for (axis, coord) in folds {
        res = fold(res, axis, coord);
    }
    res.sort();
    res.dedup();
    res
}

fn fold(coords: Vec<(u32, u32)>, fold_axis: bool, fold_coord: u32) -> Vec<(u32, u32)> {
    coords.into_iter().map(|(x,y)| {
        if fold_axis && x > fold_coord {
            (fold_coord-(x-fold_coord), y)
        } else if !fold_axis && y > fold_coord {
            (x, fold_coord-(y-fold_coord))
        } else {(x,y)}
    }).collect()
}

fn read_file(file: &str) -> (Vec<(u32, u32)>, Vec<(bool, u32)>) {
    let contents = std::fs::read_to_string(file).unwrap();
    let split = contents.split("\r\n\r\n").collect::<Vec<&str>>();
    let coord_str = split[0];
    let fold_str = split[1];
    let mut coords = vec![];
    for s in coord_str.split("\r\n") {
        let s_split = s.split(',').collect::<Vec<&str>>();
        let x = u32::from_str(s_split[0]).unwrap();
        let y = u32::from_str(s_split[1]).unwrap();
        coords.push((x,y));
    }

    let mut folds = vec![];
    for s in fold_str.split("\r\n") {
        let pre_split = s.split(" ").collect::<Vec<&str>>();
        let ready = pre_split[2];
        let s_split = ready.split("=").collect::<Vec<&str>>();
        let axis = s_split[0] == "x";
        let coords = u32::from_str(s_split[1]).unwrap();
        folds.push((axis, coords))
    }
    (coords, folds)
}
