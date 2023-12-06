use std::collections::HashSet;
use std::fs::read_to_string;

fn main() {
    let input = read_lines("input.txt");
    let mut numbers: Vec<Number> = Vec::new();
    let mut symbols: HashSet<(i32, i32)> = HashSet::new();

    let mut current_number: Option<Number> = None;

    for (row, line) in input.iter().enumerate() {
        for (col, c) in line.chars().enumerate() {
           if c.is_numeric() {
               if let Some(ref mut n) = current_number {
                   n.add_digit((c as u8 - b'0') as i32, row as i32, col as i32);
               } else {
                   current_number = Some(Number::new((c as u8 - b'0') as i32, row as i32, col as i32));
               }
           } else  {
               if let Some(n) = current_number.take() {
                   numbers.push(n);
                   current_number = None;
               }

               if c != '.' {
                   symbols.insert((row as i32, col as i32));
               }
           }
        }
    }

    if let Some(n) = current_number.take() {
        numbers.push(n);
    }

    let sum = numbers.iter()
        .filter(|n| n.surrounding_coordinates.intersection(&symbols).into_iter().next().is_some())
        .map(|n| n.value)
        .sum::<i32>();

    dbg!(sum);
}

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .unwrap()  // panic on possible file-reading errors
        .lines()  // split the string into an iterator of string slices
        .map(String::from)  // make each slice into a string
        .collect()  // gather them together into a vector
}

#[derive(Debug)]
struct Number {
    value: i32,
    surrounding_coordinates: HashSet<(i32, i32)>
}

impl Number {
    fn new(value: i32, row: i32, col: i32) -> Self {
     Self {
         value,
         surrounding_coordinates: Self::get_surrounding_coordinates(row, col)
     }
    }

    fn add_digit(&mut self, digit: i32, row: i32, col: i32) {
        self.value = self.value * 10 + digit;
        self.surrounding_coordinates.extend(Self::get_surrounding_coordinates(row, col));
    }

    fn get_surrounding_coordinates(row: i32, col: i32) -> HashSet<(i32, i32)> {
        HashSet::from([
            (row, col - 1),
            (row, col + 1),
            (row - 1, col),
            (row + 1, col),
            (row - 1, col - 1),
            (row + 1, col + 1),
            (row + 1, col - 1),
            (row - 1, col + 1)
        ])
    }
}
