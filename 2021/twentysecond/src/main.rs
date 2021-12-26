use std::collections::HashSet;
use std::cmp::{min, max};

fn main() {
    let data = read_data("test.txt");
    println!("{}", solve_2(data));
}

fn solve_1(data: Vec<Instruction>) -> usize {
    let mut res: HashSet<Cube> = HashSet::new();
    for ins in data {
        for x in  ins.from_x..=ins.to_x {
            for y in  ins.from_y..=ins.to_y {
                for z in  ins.from_z..=ins.to_z {
                    if ins.on {
                        res.insert((x,y,z));
                    } else {
                        res.remove(&(x,y,z));
                    }
                }
            }
        }
    }
    res.len()
}

fn solve_2(data: Vec<Instruction>) -> i64 {
    let mut result: Vec<Instruction> = vec![];
    for instruction in data {
        result.iter_mut().for_each(|x| x.subtract(&instruction));
        if instruction.on {
            result.push(instruction);
        }
    }
    // println!("{:?}", res);
    let count: i64 = result.iter().map(|x| x.volume()).sum::<i64>();
    count
}

// #[derive(PartialEq, Eq, Hash)]
type Cube = (i64, i64, i64);

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
struct Instruction {
    on: bool,
    from_x: i64,
    to_x: i64,
    from_y: i64,
    to_y: i64,
    from_z: i64,
    to_z: i64,
    off: Vec<Instruction>
}

fn does_line_intersect(x0: i64, x1: i64, ox0: i64, ox1: i64) -> bool {
    (x0 <= ox0 && ox0 <= x1)
        || (x0 <= ox1 && ox1 <= x1)
        || (ox0 <= x0 && x0 <= ox1)
        || (ox0 <= x1 && x1 <= ox1)
}

impl Instruction {
    fn intersect(&self, other: &Instruction) -> bool {
        let x_overlap = does_line_intersect(self.from_x, self.to_x, other.from_x, other.to_x);
        let y_overlap = does_line_intersect(self.from_y, self.to_y, other.from_y, other.to_y);
        let z_overlap = does_line_intersect(self.from_z, self.to_z, other.from_z, other.to_z);
        x_overlap && y_overlap && z_overlap
    }

    fn get_intersection(&self, other: &Instruction) -> Option<Instruction> {
        if !self.intersect(other) {
            None
        } else {
            Some(
                Instruction {
                    on: false,
                    from_x: max(self.from_x, other.from_x),
                    to_x: min(self.to_x, other.to_x),
                    from_y: max(self.from_y, other.from_y),
                    to_y: min(self.to_y, other.to_y),
                    from_z: max(self.from_z, other.from_z),
                    to_z: min(self.to_z, other.to_z),
                    off: Vec::new()
                }
            )
        }
    }

    fn subtract(&mut self, other: &Instruction) {
        if let Some(intersection) = self.get_intersection(other) {
            self.off.iter_mut().for_each(|x| x.subtract(&intersection));
            self.off.push(intersection)
        }
    }

    fn volume(&self) -> i64 {
        let start = (self.to_x - self.from_x+1)*(self.to_y-self.from_y+1)*(self.to_z-self.from_z+1);
        start - self.off.iter().map(|x| x.volume()).sum::<i64>()
    }
}

fn read_data(file: &str) -> Vec<Instruction> {
    let contents = std::fs::read_to_string(file).unwrap();
    let mut res = vec![];
    for s in contents.split("\r\n") {
        let action_split = s.split(" ").collect::<Vec<&str>>();
        let boundaries_split = action_split[1].split(",").collect::<Vec<&str>>();
        let x_name_split = boundaries_split[0].split("=").collect::<Vec<&str>>();
        let x_split = x_name_split[1].split("..").map(|s| s.parse::<i64>().unwrap()).collect::<Vec<i64>>();
        let y_name_split = boundaries_split[1].split("=").collect::<Vec<&str>>();
        let y_split = y_name_split[1].split("..").map(|s| s.parse::<i64>().unwrap()).collect::<Vec<i64>>();
        let z_name_split = boundaries_split[2].split("=").collect::<Vec<&str>>();
        let z_split = z_name_split[1].split("..").map(|s| s.parse::<i64>().unwrap()).collect::<Vec<i64>>();
        
        res.push(Instruction{
            on: action_split[0] == "on",
            from_x: x_split[0],
            to_x: x_split[1],
            from_y: y_split[0],
            to_y: y_split[1],
            from_z: z_split[0],
            to_z: z_split[1],
            off: Vec::new()
        })
    }
    res
}