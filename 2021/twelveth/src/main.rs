use std::collections::HashMap;

fn main() {
    println!("Hello, world!");
    let test = read_file("source.txt");
    println!("{:?}", solve_2(test));
}

fn solve_1(input: HashMap<String, Node>) -> u32 {
    let mut cnt = 0;
    let start = input.get("start").unwrap();
    let mut queue: Vec<Path> = vec![vec![start]];
    while !queue.is_empty() {
        let path = queue.pop().unwrap();
        for edge in path.last().unwrap().edges.iter() {
            if edge == "end" {
                cnt += 1;
            } else {
                let next_node = input.get(edge).unwrap();
                if !path.contains(&next_node) || next_node.revisitable {
                    let mut new_path = path.clone();
                    new_path.push(next_node);
                    queue.push(new_path);
                }
            }
        }
    }
    cnt
}

fn solve_2(input: HashMap<String, Node>) -> u32 {
    let mut cnt = 0;
    let start = input.get("start").unwrap();
    let first_path = PathRevisitableNode {
        path: vec![start],
        revisited: false
    };
    let mut queue: Vec<PathRevisitableNode> = vec![first_path];
    while !queue.is_empty() {
        let path = queue.pop().unwrap();
        for edge in path.path.last().unwrap().edges.iter() {
            if edge == "end" {
                cnt += 1;
            } else {
                let next_node = input.get(edge).unwrap();
                if !path.path.contains(&next_node) || next_node.revisitable {
                    let mut new_path = path.clone();
                    new_path.path.push(next_node);
                    queue.push(new_path);
                } else if path.path.contains(&next_node) && !path.revisited && edge != "start" {
                    let mut new_path = path.clone();
                    new_path.path.push(next_node);
                    new_path.revisited = true;
                    queue.push(new_path);
                }
            }
        }
    }
    cnt
}

type Path<'a> = Vec<&'a Node>;

#[derive(Clone, Debug)]
struct PathRevisitableNode<'a> {
    pub path: Path<'a>,
    pub revisited: bool
}

#[derive(PartialEq, Debug)]
struct Node {
    pub revisitable: bool,
    pub name: String,
    pub edges: Vec<String>
}

fn read_file(file: &str) -> HashMap<String, Node> {
    let contents = std::fs::read_to_string(file).unwrap();
    let mut nodes = HashMap::new();
    for edge_str in contents.split("\n") {
        let edge_prep = edge_str.trim().split("-").collect::<Vec<&str>>();
        let n1 = edge_prep[0];
        let n2 = edge_prep[1];
        if nodes.contains_key(n1) {
            let n: &mut Node = nodes.get_mut(n1).unwrap();
            n.edges.push(n2.to_string());
        } else {
            let n = Node {
                name: n1.to_string(),
                revisitable: n1 == n1.to_uppercase(),
                edges: vec![n2.to_string()]
            };
            nodes.insert(n1.to_string(), n);
        };
        let node2 = if nodes.contains_key(n2) {
            let n: &mut Node = nodes.get_mut(n2).unwrap();
            n.edges.push(n1.to_string());
        } else {
            let n = Node {
                name: n2.to_string(),
                revisitable: n2 == n2.to_uppercase(),
                edges: vec![n1.to_string()]
            };
            nodes.insert(n2.to_string(), n);
        };
    }
    nodes
}