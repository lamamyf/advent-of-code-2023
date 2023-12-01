use regex::Regex;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let input: Vec<&str> = args[1].split('\n').collect();
    let regex = Regex::new(r"\d").unwrap();
    let mut sum = 0;

    for word in input.iter(){
        let matches = regex.find_iter(word).collect::<Vec<_>>();
        if let (Some(first_match), Some(last_match)) = 
        (matches.first(), matches.last()) {
            let result = format!("{}{}", first_match.as_str(), last_match.as_str());
        
            if let Ok(parsed_int) = result.parse::<i32>() {
                sum += parsed_int;
            }
        }
    }

    dbg!(sum);
}
