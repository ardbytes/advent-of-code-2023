use std::collections::HashMap;
use regex::Regex;
use std::env;

fn remove_non_numeric(input: &str) -> String {
    let re = Regex::new(r"[^0-9|one|two|three|four|five|six|seven|eight|nine|\n]+").unwrap();
    re.replace_all(input, "").to_string()
}

fn remove_non_digits(input: &str) -> String {
    let re = Regex::new(r"[^0-9]+").unwrap();
    re.replace_all(input, "").to_string()
}

fn replace_string_with_digits(input: &str, word_to_digit: &HashMap<&str, &str>) -> String {
    let mut result = input.to_string();

    // stores first and last text representation of digits by position
    // Given "eightxtwo1three" then words will contain ["eight", "three"]
    let mut words = ["", ""];
    let mut min_index = None;
    let mut max_index = None;

    for (word, _) in word_to_digit {
        match input.find(word) {
            Some(i) => {
                if min_index.is_some() && min_index.unwrap() > i {
                    min_index = Some(i);
                    words[0] = word;
                } else if min_index.is_none() {
                    min_index = Some(i);
                    words[0] = word;
                }
                if max_index.is_some() && max_index.unwrap() < i {
                    max_index = Some(i);
                    words[1] = word;
                } else if max_index.is_none() {
                    max_index = Some(i);
                    words[1] = word;
                }
            },
            None => ()
        }
    }
    if words[0] != "" {
        result = result.replace(words[0], word_to_digit.get(words[0]).unwrap());
    }

    if words[1] != "" {
        result = result.replace(words[1], word_to_digit.get(words[1]).unwrap());
    }
    remove_non_digits(&result)
}

fn get_digits(input: &str) -> u32 {
    let first = input.chars().next().unwrap_or('0');
    let last = input.chars().last().unwrap_or(first);
    format!("{}{}", first, last).parse().unwrap_or(0)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let string = include_str!("input.txt");
    let numbers_only = remove_non_numeric(string);
    let mut sum = 0;
    if args.len() == 2 && args[1] == "part2" {
        let mut word_to_digit: HashMap<&str, &str> = HashMap::new();
        word_to_digit.insert("one", "1");
        word_to_digit.insert("two", "2");
        word_to_digit.insert("three", "3");
        word_to_digit.insert("four", "4");
        word_to_digit.insert("five", "5");
        word_to_digit.insert("six", "6");
        word_to_digit.insert("seven", "7");
        word_to_digit.insert("eight", "8");
        word_to_digit.insert("nine", "9");

        for line in numbers_only.split("\n") {
            let result = replace_string_with_digits(line, &word_to_digit);
            sum += get_digits(&result);
        }
    } else {
        for line in numbers_only.split("\n") {
            let result = remove_non_digits(&line);
            sum += get_digits(&result);
        }
    }
    println!("{}", sum);
}


