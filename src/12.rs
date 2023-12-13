use regex::Regex;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let regex = Regex::new(r"\d[ \d]*").unwrap();

    let vector = regex.find_iter(&args[1])
        .map(|s| s.as_str().split_whitespace().filter_map(|num| num.parse::<i64>().ok()).collect::<Vec<i64>>()) // Take the first two vectors
        .collect::<Vec<Vec<i64>>>();

    let time = digit_vector_to_int(vector.first().unwrap());
    let distance = digit_vector_to_int(vector.last().unwrap());

    let mut result: Vec<i64>  = Vec::new();
    let mut margin = 0;
    for j in 1..time {
        if (j * (time - j)) >= distance {
            margin += 1;
        }
    }

    result.push(margin);

    dbg!(result.iter().product::<i64>());
}

fn digit_vector_to_int(vector: &Vec<i64>) -> i64 {
    vector.iter().fold(0, |acc, &digit| {
        let digit_power = 10_i64.pow((digit).ilog10() + 1);
        acc * digit_power + digit
    })
}