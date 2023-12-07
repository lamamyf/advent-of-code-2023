use std::env;
use std::collections::HashSet;
use fancy_regex::Regex;

fn main() {
    let args: Vec<String> = env::args().collect();
    let input: Vec<&str> = args[1].split('\n').collect();
    let regex = Regex::new(r"(?<=:|\|)\s+[\d+ ]+\d+").unwrap();
    let mut sum = 0;

    for word in input.iter(){
        let vector: Vec<HashSet<String>> = regex.find_iter(word)
            .flat_map(|s| s.ok())
            .map(|s| s.as_str())
            .map(|s| s.trim())
            .map(|s| HashSet::from_iter(s.split_whitespace().map(String::from)))
            .collect();

        if let (Some(first), Some(last)) = (vector.first(), vector.last()) {
            let len = (first.intersection(last).collect::<Vec<&String>>().len()) as u32;
            if len > 0 {
                sum += 2_u32.pow(len - 1);
            }
        }
    }

    dbg!(sum);
}