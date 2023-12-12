use fancy_regex::Regex;
use std::env;
use std::ops::Range;
use std::cmp::{max, min};

fn main() {
    let args: Vec<String> = env::args().collect();

    let seeds_regex = Regex::new(r"(?<=seeds:)[\d+ ]+\d+").unwrap();
    let maps_regex = Regex::new(r"(?<= map:\n)([\d+ ]+\d+\n)*").unwrap();

    let seed_ranges: Vec<Range<i64>> = seeds_regex
        .find_iter(&args[1])
        .flat_map(|s| s.ok())
        .map(|s| s.as_str())
        .flat_map(|line| {
            line.trim()
                .split(' ')
                .flat_map(|s| s.parse::<i64>().ok())
        })
        .collect::<Vec<i64>>()
        .chunks_exact(2)
        .filter_map(|chunk| {
            if chunk.len() == 2 {
                Some(chunk[0]..(chunk[0] + chunk[1]))
            } else {
                None
            }
        }).collect();



    let mut result = seed_ranges;

    maps_regex.find_iter(&args[1]).for_each( |soil| {
        let soil_vector: Vec<Vec<i64>> = soil.iter()
            .flat_map(|s| s.as_str().split('\n'))
            .filter(|line| !line.trim().is_empty())
            .map(|line| line.split(' ')
                .map(String::from)
                .map(|s: String| s.parse::<i64>())
                .flat_map(|s| s.ok())
                .collect())
            .collect();

        result = find_values(&mut result, soil_vector);
    });

    dbg!(&result.iter().map(|r| r.start).min());
}

fn find_values(ranges: &mut Vec<Range<i64>>, map_values: Vec<Vec<i64>>) -> Vec<Range<i64>> {
    let mut result: Vec<Range<i64>>  = Vec::new();

    while ranges.len() > 0 {
        let range = ranges.pop().unwrap();
        let mut in_range = false;

        map_values.iter().find(|value| {
            let des_range = value[0]..(value[0] + value[2]);
            let src_range = value[1]..(value[1] + value[2]);
            let overlap = max(range.start, src_range.start)..min(range.end, src_range.end);
            if overlap.start < overlap.end {
                result.push((overlap.start - src_range.start +  des_range.start)..(overlap.end - src_range.start +  des_range.start));


                if overlap.start > range.start {
                    ranges.push(range.start..overlap.start);
                }

                if range.end > overlap.end {
                    ranges.push(overlap.end..range.end);
                }

                in_range = true;
                true
            } else {
                false
            }
        });

        if !in_range {
            result.push(range.clone());
        }
    }

    result
}