use std::env;
use regex::Regex;

fn main() {
    let args: Vec<String> = env::args().collect();
    let input: Vec<&str> = args[1].split('\n').collect();

    let red_regex = Regex::new(r"\d+ red").unwrap();
    let green_regex = Regex::new(r"\d+ green").unwrap();
    let blue_regex = Regex::new(r"\d+ blue").unwrap();

    let mut sum = 0;

    for line in input.iter() {    
        let max_red = get_max(red_regex.find_iter(line).map(|m| m.as_str()).collect()); 
        let max_green = get_max(green_regex.find_iter(line).map(|m| m.as_str()).collect());
        let max_blue = get_max(blue_regex.find_iter(line).map(|m| m.as_str()).collect()); 

        sum += max_red * max_green * max_blue;
    }

    dbg!(sum);
}

fn get_max(matches: Vec<&str>) -> i32 {
    *matches.iter()
        .map(|m| m.chars().take(m.find(" ").unwrap()).collect())
        .map(|m: String| m.parse::<i32>())
        .flat_map(|m| m.ok())
        .collect::<Vec<i32>>()
        .iter()
        .max()
        .unwrap()
}

