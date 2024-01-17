use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::process;

pub enum Day1Part {
    One,
    Two
}

pub fn day_1(part: Day1Part) {
    println!("reading file for inputs");

    let path = "./test-inputs/day1/day1-part-2.txt";
    let file = match File::open(path) {
        Ok(file) => file,
        Err(_) => {
            eprintln!("unable to read file");
            process::exit(1);
        }
    };

    let reader = BufReader::new(file);
    let mut sum = 0;
    let rg;

    match part {
        Day1Part::One => {
            rg = Regex::new(r"\d+").unwrap();
            for result in reader.lines() {
                let line = result.unwrap();
                sum = sum + get_num_digits(&line, &rg);
            }
        }
        Day1Part::Two => {
            for result in reader.lines() {
                let line = result.unwrap();
                sum = sum + get_word_digits(&line);
            }
        }
    }

    println!("total sum: {sum}")
}

fn extract_digit(digits: &HashMap<char, Vec<&str>>, ch: char, line: &String, i: usize) -> String {
    let nums = [
        "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    let vals = digits.get(&ch).unwrap();
    let start_index = i;
    let line_lenght = line.chars().count();

    for val in vals.iter() {
        let end_index = start_index + val.chars().count();

        if start_index == line_lenght - 1 {
            break;
        }

        if end_index > line_lenght {
            break;
        }

        let slice = &line[start_index..end_index];

        if slice.to_string() == val.to_string() {
            for (index, num) in nums.iter().enumerate() {
                if slice.to_string() == num.to_string() {
                    return index.to_string();
                }
            }
        }
    }

    return "".to_string();
}

fn get_word_digits(line: &String) -> i32 {
    let mut sets: Vec<String> = Vec::new();
    let digits = HashMap::from([
        ('o', vec!["one"]),
        ('t', vec!["two", "three"]),
        ('f', vec!["four", "five"]),
        ('s', vec!["six", "seven"]),
        ('e', vec!["eight"]),
        ('n', vec!["nine"]),
        ('z', vec!["zero"]),
    ]);

    line.chars().enumerate().for_each(|(i, ch)| {
        if ch.is_digit(10) {
            sets.push(ch.to_string());
        }

        let valid_char = ['o', 't', 'f', 's', 'e', 'n', 'z'];
        if valid_char.contains(&ch) {
            let word = extract_digit(&digits, ch, line, i);
            if !word.is_empty() {
                sets.push(word);
            }
        }
    });

    let join = format!("{}{}", sets.first().unwrap(), sets.last().unwrap());
    return join.parse::<i32>().unwrap();
}

fn get_num_digits(line: &String, rg: &Regex) -> i32 {
    let mut digits = vec!["", ""];

    line.split("").for_each(|char| {
        if rg.is_match(char) {
            if digits[0].is_empty() {
                digits[0] = char;
            } else {
                digits[1] = char;
            }
        }
    });

    if digits[1].is_empty() {
        digits[1] = digits[0];
    }

    let digits_sum = digits.join("").parse::<i32>().unwrap();

    return digits_sum;
}
