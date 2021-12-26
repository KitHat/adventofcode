fn main() {
    let data = read_content("test.txt");
    println!("res 2 {}", solve_2(&data));
    println!("res {}", solve_1(data));
}

fn solve_2(data: &Vec<Node>) -> u64 {
    let mut max_magnitude = 0;
    for i in 0..data.len() {
        for j in 0..data.len() {
            // if i == j {continue}
            let a = data[i].clone();
            let b = data[j].clone();
            let mut sum1 = a.add(b);
            while sum1.reduce() {}
            let magn1 = sum1.magnitude();
            if magn1 > max_magnitude {max_magnitude = magn1}
        }
    }
    max_magnitude
}

fn solve_1(mut data: Vec<Node>) -> u64 {
    let mut sum = data.remove(0);
    while !data.is_empty() {
        let next = data.remove(0);
        sum = sum.add(next);
        // sum.print();
        // println!();
        while sum.reduce() {
            // sum.print();
            // println!();
        }
        // println!("END");
    }
    sum.print();
    sum.magnitude()
}

#[derive(Clone, Debug)]
struct Node {
    content: NodeContent
}

#[derive(Clone, Debug)]
enum Action {
    AddRight(u64),
    AddLeft(u64),
    Finish
}

impl Node {
    pub fn add(self, to_add: Node) -> Node {
        Node {
            content: NodeContent::Pair(Box::new(self), Box::new(to_add))
        }
    }

    fn explode(&mut self, level: usize) -> Vec<Action> {
        let (node_content, actions) = match &mut self.content {
            NodeContent::Val(_) => (None, vec![]),
            NodeContent::Pair(left, right) => {
                if level > 3 {
                    let left_val = match left.content {
                        NodeContent::Val(x) => x,
                        _ => panic!("LEVEL 5")
                    };
                    let right_val = match right.content {
                        NodeContent::Val(x) => x,
                        _ => panic!("LEVEL 5")
                    };
                    (Some(NodeContent::Val(0)), vec![Action::AddLeft(left_val), Action::AddRight(right_val)])
                } else {
                    (None, vec![])
                }
            }
        };
        if let Some(c) = node_content {
            self.content = c;
            actions
        } else {
            match &mut self.content {
                NodeContent::Val(_) => vec![],
                NodeContent::Pair(left, right) => {
                    let next_actions = left.explode(level+1);
                    let mut reactions: Vec<Action> = vec![];
                    for action in next_actions {
                        match action {
                            Action::Finish => reactions.push(action.clone()),
                            Action::AddLeft(_) => reactions.push(action),
                            Action::AddRight(x) => {
                                right.add_val(x, false);
                                reactions.push(Action::Finish)
                            }
                        }
                    }
                    if reactions.is_empty() {
                        // println!("going right {}", level+1);
                        let next_actions = right.explode(level+1);
                        for action in next_actions {
                            match action {
                                Action::Finish => reactions.push(action.clone()),
                                Action::AddRight(_) => reactions.push(action),
                                Action::AddLeft(x) => {
                                    left.add_val(x, true);
                                    reactions.push(Action::Finish)
                                }
                            }
                        }
                    }
                    reactions
                }
            }
        }
    }

    fn split(&mut self, level: usize) -> Vec<Action> {
        let (node_content, actions) = match &mut self.content {
            NodeContent::Val(x) => {
                if *x >= 10 {
                    if level > 3 {
                        (Some(NodeContent::Val(0)), vec![Action::AddLeft(*x/2), Action::AddRight(*x/2+*x%2)])
                    } else {
                        (Some(NodeContent::Pair(
                            Box::new(Node {
                                content: NodeContent::Val(*x/2)
                            }), Box::new(Node {
                                content: NodeContent::Val(*x/2+*x%2)
                        }))), vec![Action::Finish])
                    }
                } else {
                    (None, vec![])
                }
            },
            NodeContent::Pair(_, _) => (None, vec![])
        };
        
        if let Some(c) = node_content {
            self.content = c;
            actions
        } else {
            match &mut self.content {
                NodeContent::Val(_) => vec![],
                NodeContent::Pair(left, right) => {
                    let next_actions = left.split(level+1);
                    let mut reactions: Vec<Action> = vec![];
                    for action in next_actions {
                        match action {
                            Action::Finish => reactions.push(action.clone()),
                            Action::AddLeft(_) => reactions.push(action),
                            Action::AddRight(x) => {
                                right.add_val(x, false);
                                reactions.push(Action::Finish)
                            }
                        }
                    }
                    if reactions.is_empty() {
                        // println!("going right {}", level+1);
                        let next_actions = right.split(level+1);
                        for action in next_actions {
                            match action {
                                Action::Finish => reactions.push(action.clone()),
                                Action::AddRight(_) => reactions.push(action),
                                Action::AddLeft(x) => {
                                    left.add_val(x, true);
                                    reactions.push(Action::Finish)
                                }
                            }
                        }
                    }
                    reactions
                }
            }
        }
    }

    pub fn reduce(&mut self) -> bool {
        let actions = self.explode(0);
        if !actions.is_empty() {
            return true;
        }
        let actions = self.split(0);
        !actions.is_empty()
    }

    fn add_val(&mut self, val: u64, dir: bool) {
        let new_content = match &mut self.content {
            NodeContent::Val(x) => Some(NodeContent::Val(*x+val)),
            NodeContent::Pair(left, right) => {
                if dir {
                    right.add_val(val, dir)
                } else {
                    left.add_val(val, dir)
                }
                None
            }
        };
        if let Some(c) = new_content {
            self.content = c
        }
    }

    pub fn magnitude(&self) -> u64 {
        match &self.content {
            NodeContent::Val(x) => *x,
            NodeContent::Pair(left, right) => left.magnitude()*3 + right.magnitude()*2
        }
    }

    pub fn parse(input: &mut Vec<char>) -> Node {
        let c = input.remove(0);
        let content = match c.clone() {
            '[' => {
                let left = Node::parse(input);
                input.remove(0); //comma
                let right = Node::parse(input);
                input.remove(0); //closing brace
                NodeContent::Pair(Box::new(left), Box::new(right))
            },
            _ => NodeContent::Val(c.to_digit(10).unwrap().into())
        };
        Node {
            content
        }
    }

    pub fn print(&self) {
        match &self.content {
            NodeContent::Val(x) => print!("{}",x),
            NodeContent::Pair(l,r) => {
                print!("[");
                l.print();
                print!(",");
                r.print();
                print!("]");
            }
        }
    }
}

#[derive(Clone, Debug)]
enum NodeContent {
    Val(u64),
    Pair(Box<Node>, Box<Node>) //left, right
}

fn read_content(file: &str) -> Vec<Node> {
    let contents = std::fs::read_to_string(file).unwrap();
    contents.split("\r\n").map(|s| {
        let mut chars = s.chars().collect::<Vec<char>>();
        Node::parse(&mut chars)
    }).collect()
}


