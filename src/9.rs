use fancy_regex::Regex;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let seeds_regex = Regex::new(r"(?<=seeds:)[\d+ ]+\d+").unwrap();
    let map_regex = Regex::new(r"(?<= map:\n)([\d+ ]+\d+\n)*").unwrap();

    let seeds: Vec<i64> = seeds_regex
        .find_iter(&args[1])
        .flat_map(|s| s.ok())
        .map(|s| s.as_str())
        .flat_map(|line| {
            line.trim()
                .split(' ')
                .flat_map(|s| s.parse::<i64>().ok())
        })
        .collect();

    let mut result = seeds;

    map_regex.find_iter(&args[1]).for_each( |soil| {
        let soil_vector: Vec<Vec<i64>> = soil.iter()
            .flat_map(|s| s.as_str().split('\n'))
            .filter(|line| !line.trim().is_empty())
            .map(|line| line.split(' ')
                .map(String::from)
                .map(|s: String| s.parse::<i64>())
                .flat_map(|s| s.ok())
                .collect())
            .collect();

        result = x(&result, soil_vector);
    });

    dbg!(&result.iter().min());
}

fn x(keys: &Vec<i64>, map: Vec<Vec<i64>>) -> Vec<i64> {
    let mut result: Vec<i64>  = Vec::new();

    keys.iter().for_each(|key| {
        let mut in_range = false;
        map.iter().for_each(|value| {
                    let mut des_range = value[0]..(value[0] + value[2]);
                    let mut src_range = value[1]..(value[1] + value[2]);

                    if let Some(index) = src_range.position(|r| r == *key) {
                        result.push(des_range.nth(index).unwrap());
                        in_range = true;
                    }
                });

                if !in_range {
                    result.push(*key);
                }
            });

    result
}