use std::env;
use regex::Regex;

fn main() {
    let args: Vec<String> = env::args().collect();
    let input: Vec<&str> = args[1].split('\n').collect();

    let red_regex = Regex::new(r"\d+ red").unwrap();
    let green_regex = Regex::new(r"\d+ green").unwrap();
    let blue_regex = Regex::new(r"\d+ blue").unwrap();

    let red_cubes = 12;
    let green_cubes = 13;
    let blue_cubes = 14;

    let mut sum = 0;

    for (i, line) in input.iter().enumerate() {    
        if !check_cubes(red_regex.find_iter(line).map(|m| m.as_str()).collect(), red_cubes) { 
            continue; 
        }
        
        if !check_cubes(green_regex.find_iter(line).map(|m| m.as_str()).collect(), green_cubes) { 
            continue; 
        }

        if !check_cubes(blue_regex.find_iter(line).map(|m| m.as_str()).collect(), blue_cubes) { 
            continue; 
        }

        sum += i+1;
    }

    dbg!(sum);
}

fn check_cubes(matches: Vec<&str>, number: i32) -> bool {
    matches.iter()
        .map(|m| m.chars().take(m.find(" ").unwrap()).collect())
        .map(|m: String| m.parse::<i32>())
        .flat_map(|m| m.ok())
        .filter(|m| m > &number)
        .collect::<Vec<i32>>()
        .capacity() == 0
}
