use std::collections::{HashMap, HashSet};

fn main() {
    println!("Hello, world!");
    let test = input("test");
    let source = input("source");
    // println!("test 1 {}", solve_1(&test));
    // println!("source 1 {}", solve_1(&source));
    println!("test 2 {}", solve_2(&test));
    println!("source 2 {}", solve_2(&source));
} 

fn solve_1(input: &Vec<String>) -> u32 {
    input.iter().map(|s| {
        let a = validate(s);
        // println!("{}", a);
        a
    }).sum()
}

fn solve_2(input: &Vec<String>) -> u64 {
    let mut validations: Vec<u64> = input.iter().map(|s| validate_incomplete(s)).filter(|a| *a != 0).collect();
    validations.sort();
    validations[validations.len()/2]
}

fn validate(s: &str) -> u32 {
    let rewards: HashMap<char, u32> = HashMap::from([(')', 3), (']', 57), ('}', 1197), ('>', 25137)]);
    let opens: HashSet<char> = HashSet::from(['(', '[', '{', '<']);
    let matches: HashMap<char, char> = HashMap::from([('(', ')'), ('[', ']'), ('{', '}'), ('<', '>')]);
    let mut validator: Vec<char> = vec![];
    for c in s.chars() {
        if opens.contains(&c) {
            validator.push(c)
        } else {
            let matching_bracket = validator.pop();
            if let Some(mb) = matching_bracket {
                if *matches.get(&mb).unwrap() != c {
                    return rewards.get(&c).unwrap().clone();
                }
            } else {
                return rewards.get(&c).unwrap().clone();
            }
        }
    }
    0
}

fn validate_incomplete(s: &str) -> u64 {
    let rewards: HashMap<char, u64> = HashMap::from([('(', 1), ('[', 2), ('{', 3), ('<', 4)]);
    let opens: HashSet<char> = HashSet::from(['(', '[', '{', '<']);
    let matches: HashMap<char, char> = HashMap::from([('(', ')'), ('[', ']'), ('{', '}'), ('<', '>')]);
    let mut validator: Vec<char> = vec![];
    for c in s.chars() {
        if opens.contains(&c) {
            validator.push(c)
        } else {
            let matching_bracket = validator.pop();
            if let Some(mb) = matching_bracket {
                if *matches.get(&mb).unwrap() != c {
                    return 0;
                }
            } else {
                return 0;
            }
        }
    }
    validator.reverse();
    validator.iter().map(|c| rewards.get(c).unwrap()).fold(0, |acc, next| {acc*5 + next})
}

fn input(filename: &str) -> Vec<String> {
    let contents: String = std::fs::read_to_string(filename).unwrap();
    contents.split("\n").map(|s| s.trim().to_string()).collect::<Vec<String>>()
}