use regex::Regex;
use std::env;
use std::mem::take;

fn main() {
    let args: Vec<String> = env::args().collect();
    let regex = Regex::new(r"\d[ \d]*").unwrap();

    let vector = regex.find_iter(&args[1])
        .map(|s| s.as_str().split_whitespace().filter_map(|num| num.parse::<i64>().ok()).collect::<Vec<i64>>()) // Take the first two vectors
        .collect::<Vec<Vec<i64>>>();

    let times: &Vec<i64> =  vector.first().unwrap();
    let distances: &Vec<i64> =  vector.last().unwrap();

    let mut result: Vec<i64>  = Vec::new();

    for (i, time) in times.iter().enumerate()  {
        let mut margin = 0;
        for j in 1..*time {
            if (j * (time - j)) >= distances[i] {
                margin += 1;
            }
        }

        result.push(margin);
    }

    dbg!(result.iter().product::<i64>());
}

