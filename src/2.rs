use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let input: Vec<&str> = args[1].split('\n').collect();
    
    let mut sum = 0;

    for word in input.iter(){
               let numbers = find_numbers(word);
               if let (Some(num1), Some(num2)) = (numbers.first(), numbers.last()) {
               let result = format!("{}{}", num1.to_string(), num2.to_string());
               if let Ok(parsed_int) = result.parse::<i32>() {
                   sum += parsed_int;
               }
             }
    }

    dbg!(sum);
}

fn find_numbers(s: &str) -> Vec<i32> {
    let words = vec![
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ];

    let mut numbers = Vec::new();
    let chars: Vec<char> = s.chars().collect();

    for i in 0..chars.len() {
        if let Some(digit) = chars[i].to_digit(10) {
            numbers.push(digit as i32);
        } else {
            for (word, num) in words.iter() {
                let word_chars: Vec<char> = word.chars().collect();
                if chars[i..].starts_with(&word_chars) {
                    numbers.push(*num);
                }
            }
        }
    }

    numbers
}
