use std::collections::HashMap;

fn main() {
    println!("Hello, world!");
    let (test_start, test_rules) = read_file("test.txt");
    println!("test {}", solve_1(&test_start, &test_rules));
    
    let (source_start, source_rules) = read_file("source.txt");
    println!("source {}", solve_1(&source_start, &source_rules));
}

fn solve_1(start: &str, rules: &HashMap<String, char>) -> u64 {
    let mut occurences: HashMap<char, u64> = start.chars().fold(HashMap::new(), |mut acc, next| {*acc.entry(next).or_insert(0) += 1; acc});
    let mut pair_occurences: HashMap<String, u64> = rules.iter().map(|(s, _)| (s.clone(), 0)).collect();
    // println!("step 0: {:?}", occurences);
    for i in 0..start.len() - 1 {
        let key = &start[i..i+2];
        *pair_occurences.entry(key.to_string()).or_insert(0) += 1;
    }
    // println!("step 0: {:?}", pair_occurences);
    for _step in 0..40 {
        let mut next = pair_occurences.clone();
        for (key, value) in pair_occurences.iter() {
            if *value == 0 {
                continue;
            }
            *next.entry(key.clone()).or_insert(*value) -= value;
            let produced = rules.get(key).unwrap().clone();
            *occurences.entry(produced).or_insert(0) += value;
            let mut start: String = key[0..1].to_string().clone();
            start.insert(1, produced);
            *next.entry(start).or_insert(0) += value;
            let mut end = key[1..].to_string().clone();
            end.insert(0, produced);
            *next.entry(end).or_insert(0) += value;
        } 
        pair_occurences = next;
        // println!("step {}: {:?}", _step+1, pair_occurences);
        // println!("step {}: {:?}", _step+1, occurences);
    }
    let (_, max) = occurences.iter().max_by(|a, b| a.1.cmp(b.1)).unwrap();
    let (_, min) = occurences.iter().min_by(|a, b| a.1.cmp(b.1)).unwrap();
    max-min
}

fn read_file(file: &str) -> (String, HashMap<String, char>) {
    let contents = std::fs::read_to_string(file).unwrap();
    let start_rules_split = contents.split("\r\n\r\n").collect::<Vec<&str>>();
    let start = start_rules_split[0].to_string();
    let mut rules = HashMap::new();
    for rule in start_rules_split[1].split("\r\n") {
        let in_out_split = rule.split(" -> ").collect::<Vec<&str>>();
        rules.insert(in_out_split[0].to_string(), in_out_split[1].chars().nth(0).unwrap());
    }
    (start, rules)
}