fn main() {
    println!("Hello, world!");
    let data = read_file("test.txt");
    println!("res {}", solve_1(data));
}

fn solve_1(input: Vec<Scanner>) -> i32 {
    let mut unused = (1..input.len()).collect::<std::collections::HashSet::<usize>>();
    let mut cur_i = 0;
    let mut cur = &input[cur_i];
    let mut next_i: Vec<usize> = vec![];
    let mut res = std::collections::HashSet::<Beacon>::new();
    for b in &cur.beacons {
        res.insert(b.clone());
    }
    let mut transformations: Vec<Vec<Transformation>> = vec![vec![]; input.len()];
    while !unused.is_empty() {
        println!("looking to cur {}", cur_i);
        println!("left {:?}", unused);
        let mut collected = vec![];
        let base_transform = transformations[cur_i].clone();
        for i in &unused {
            // println!("checking {}", i);
            if let Some((rotation, movement)) = cur.commons(&input[*i]) {
                println!("found {}", i);
                collected.push(*i);
                let mut base = base_transform.clone();
                base.push(Transformation::Movement(movement));
                base.push(Transformation::Rotation(rotation));
                transformations[*i] = base;
            }
        }
        for i in &collected {
            next_i.push(*i);
            let in_zero = transformations[*i].iter().rev().fold(input[*i].clone(), |acc, next| acc.transform(next));
            for b in in_zero.beacons {
                res.insert(b);
            }
        }
        // collected.reverse();
        for i in &collected {
            unused.remove(i);
        }
        cur_i = if let Some(i) = next_i.pop() {
            i
        } else {break};
        cur = &input[cur_i];
    }
    println!("number: {}", res.len());
    let a = transformations.iter().map(|v| v.iter().rev().fold(Beacon{x:0, y:0, z:0}, |acc, next| match next {
        Transformation::Rotation(r) => acc.rotate(r.clone()),
        Transformation::Movement(n) => acc.clone()+n.clone()
    })).collect::<Vec<Beacon>>();
    println!("{:?}", a);
    let mut res = 0;
    for i in 0..a.len() {
        for j in i+1..a.len() {
            let temp = a[i].clone()-a[j].clone();
            let temp_dist = temp.x.abs() + temp.y.abs() + temp.z.abs();
            if temp_dist > res {
                res = temp_dist
            }
        }
    }
    res
}

#[derive(Clone)]
enum Transformation {
    Rotation(i32),
    Movement(Beacon)
}

#[derive(Clone)]
struct Scanner {
    beacons: Vec<Beacon>
}

impl Scanner {
    fn rotate(&self, rotation: i32) -> Scanner {
        let beacons: Vec<Beacon> = self.beacons.iter().map(|b| b.rotate(rotation)).collect();
        Scanner {beacons}
    }

    fn commons(&self, other: &Scanner) -> Option<(i32, Beacon)> {
        let mut res_map = std::collections::HashMap::<(i32, Beacon), Vec<(Beacon, Beacon)>>::new();
        for r in 0..24 {
            let rotated = other.rotate(r);
            for b1 in &self.beacons {
                for b2 in &rotated.beacons {
                    let dif = b1.clone()-b2.clone();
                    // println!("a {:?} b {:?} c {:?}", b1, b2, dif);
                    res_map.entry((r, dif)).or_insert(vec![]).push((b1.clone(), b2.clone()));
                }
            }
        }
        // res_map.iter().filter(|(_,v)| v.len() >= 3).for_each(|(k, v)| println!("{:?} => {:?}", k, v));
        
        res_map.iter().filter(|(_,v)| v.len() >= 12).map(|(k,_)| k.clone()).nth(0)
    }

    fn drag(&self, movement: &Beacon) -> Scanner{
        let beacons: Vec<Beacon> = self.beacons.iter()
            .map(|b| b.clone() + movement.clone()).collect();
        Scanner {beacons}
    }

    fn transform(&self, transformation: &Transformation) -> Scanner {
        match transformation {
            Transformation::Rotation(r) => self.rotate(*r),
            Transformation::Movement(m) => self.drag(m)
        }
    }
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
struct Beacon {
    x: i32,
    y: i32,
    z: i32
}

impl Beacon {
    fn rotate(&self, rotation: i32) -> Beacon {
        match rotation {
            0  => Beacon { x: self.x, y: self.y, z: self.z},
            1  => Beacon { x: self.y, y:-self.x, z: self.z},
            2  => Beacon { x:-self.x, y:-self.y, z: self.z},
            3  => Beacon { x:-self.y, y: self.x, z: self.z},
            4  => Beacon { x: self.z, y: self.y, z:-self.x},
            5  => Beacon { x: self.y, y:-self.z, z:-self.x},
            6  => Beacon { x:-self.z, y:-self.y, z:-self.x},
            7  => Beacon { x:-self.y, y: self.z, z:-self.x},
            8  => Beacon { x: self.z, y:-self.x, z:-self.y},
            9  => Beacon { x:-self.x, y:-self.z, z:-self.y},
            10 => Beacon { x:-self.z, y: self.x, z:-self.y},
            11 => Beacon { x: self.x, y: self.z, z:-self.y},
            12 => Beacon { x: self.z, y:-self.y, z: self.x},
            13 => Beacon { x:-self.y, y:-self.z, z: self.x},
            14 => Beacon { x:-self.z, y: self.y, z: self.x},
            15 => Beacon { x: self.y, y: self.z, z: self.x},
            16 => Beacon { x: self.z, y: self.x, z: self.y},
            17 => Beacon { x: self.x, y:-self.z, z: self.y},
            18 => Beacon { x:-self.z, y:-self.x, z: self.y},
            19 => Beacon { x:-self.x, y: self.z, z: self.y},
            20 => Beacon { x:-self.x, y: self.y, z:-self.z},
            21 => Beacon { x: self.y, y: self.x, z:-self.z},
            22 => Beacon { x: self.x, y:-self.y, z:-self.z},
            23 => Beacon { x:-self.y, y:-self.x, z:-self.z},
            _ => unreachable!()
          }
    }
}

impl std::ops::Sub for Beacon {
    type Output = Beacon;
    fn sub(self, rhs: Beacon) -> Beacon {
        Beacon {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z
        }
    }
}

impl std::ops::Add for Beacon {
    type Output = Beacon;
    fn add(self, rhs: Beacon) -> Beacon {
        Beacon {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z
        }
    }
}

fn read_file(file: &str) -> Vec<Scanner> {
    let contents = std::fs::read_to_string(file).unwrap();
    let mut res = vec![];
    for sc in contents.split("\r\n\r\n") {
        let mut beacons = vec![];
        for s in sc.split("\r\n").skip(1) {
            let coords: Vec<i32> = s.split(",").map(|s| s.parse::<i32>()).flatten().collect();
            beacons.push(Beacon {
                x: coords[0],
                y: coords[1],
                z: coords[2]
            })
        }
        res.push(Scanner{beacons});
    }
    res
}
