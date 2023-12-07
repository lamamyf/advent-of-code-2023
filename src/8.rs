use std::env;
use std::collections::HashSet;
use fancy_regex::Regex;

fn main() {
    let args: Vec<String> = env::args().collect();
    let input: Vec<&str> = args[1].split('\n').collect();
    let regex = Regex::new(r"(?<=:|\|)\s+[\d+ ]+\d+").unwrap();

    let mut scratchcards = [1; 198];

    for (i,word) in input.iter().enumerate(){
        let vector: Vec<HashSet<String>> = regex.find_iter(word)
            .flat_map(|s| s.ok())
            .map(|s| s.as_str())
            .map(|s| s.trim())
            .map(|s| HashSet::from_iter(s.split_whitespace().map(String::from)))
            .collect();

        if let (Some(first), Some(last)) = (vector.first(), vector.last()) {
            let len = first.intersection(last).collect::<Vec<&String>>().len();
            if len > 0 {
                for _ in 0..scratchcards[i] {
                    for a in (i + 1)..=(len + i) {
                        scratchcards[a] += 1;
                    }
                }
            }
        }
    }

    let sum = scratchcards.iter().sum::<i32>();
    dbg!(sum);
}