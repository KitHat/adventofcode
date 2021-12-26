fn main() {
    println!("1 test {:?}", solve_1(test()));
    println!("1 source {:?}", solve_1(source()));
    
}

fn solve_1(a: Area) -> (i64, i64) {
    let mut max_y = 0;
    let mut cnt = 0;
    for x_s in 0..156 {
        for y_s in -102..1200 {
            let (new_y, hit) = eval(x_s, y_s, &a);
            max_y = std::cmp::max(max_y, new_y);
            if hit {cnt +=1;}
        }
    }
    (max_y, cnt)
}

fn eval(x_s: i64, y_s: i64, a: &Area) -> (i64, bool) {
    let mut xs = x_s;
    let mut ys = y_s;
    let mut c = Coord {x:0, y:0};
    let mut max_y = 0;
    let mut hit_b = false;
    while !overshoot(&c, a) {
        //first increase coord
        c.x += xs;
        c.y += ys;
        //then decrease speed
        if xs != 0 {
            xs -= 1;
        }
        ys -= 1;
        //check the max y
        max_y = std::cmp::max(max_y, c.y);
        //check the hit
        hit_b = hit_b || hit(&c, a);
    }
    if hit_b {
        (max_y, true)
    } else {
        (0, false)
    }
}

fn hit(c: &Coord, a: &Area) -> bool {
    c.x >= a.x1 && c.x <= a.x2 && c.y >= a.y1 && c.y <= a.y2
}

fn overshoot(c: &Coord, a: &Area) -> bool {
    c.x > a.x2 || c.y < a.y1
}

struct Coord {
    x: i64,
    y: i64
}

struct Area {
    x1: i64,
    x2: i64,
    y1: i64,
    y2: i64
}

fn test() -> Area {
    Area {
        x1: 20,
        x2: 30,
        y1: -10,
        y2: -5
    }
}

fn source() -> Area {
    Area {
        x1: 135,
        x2: 155,
        y1: -102,
        y2: -78
    }
}
