use std::env;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Read;
use std::path::Path;
use std::collections::HashMap;

fn open_input(filename: &str) -> io::Result<File> {
    let path = Path::new(filename);
    File::open(path)
}

fn read_input(reader: impl Read) -> Result<Vec<String>, String> {
    let reader = BufReader::new(reader);

    let mut output = Vec::new();
    for line_iter in reader.lines() {
        match line_iter {
            Ok(x) => output.push(x),
            Err(x) => {
                return Err(format!("cannot read input: {:?}", x));
            }
        }
    }

    Ok(output)
}

fn get_spelled_calibration_value(line: &str) -> i64 {
    let digit_map: HashMap<&str, &str> = [
        ("one", "1"), ("two", "2"), 
        ("three", "3"), ("four", "4"), ("five", "5"), 
        ("six", "6"), ("seven", "7"), ("eight", "8"), 
        ("nine", "9"),
    ].iter().cloned().collect();

    let mut first_digit = None;
    let mut last_digit = None;
    let mut i = 0;

    while i < line.len() {
        let remaining = &line[i..];
        let mut found_digit = None;

        for (word, &num) in &digit_map {
            if remaining.starts_with(word) {
                found_digit = Some(num.to_string());
                break;
            }
        }

        if found_digit.is_none() {
            if remaining.chars().next().unwrap().is_digit(10) {
                found_digit = Some(remaining.chars().next().unwrap().to_string());
            }
        }

        if let Some(digit) = found_digit {
            if first_digit.is_none() {
                first_digit = Some(digit.clone());
            }
            last_digit = Some(digit);
        }

        i += 1;
    }

    let value = first_digit.unwrap().parse::<i64>().unwrap() * 10 + last_digit.unwrap().parse::<i64>().unwrap();

    value
}

fn get_calibration_value(line: &str) -> i64 {
    let digits: Vec<char> = line.chars().filter(|c| c.is_digit(10)).collect();
    if digits.is_empty() {
        return 0;
    }
    let first_digit = digits.first().unwrap().to_digit(10).unwrap();
    let last_digit = digits.last().unwrap().to_digit(10).unwrap();
    (first_digit * 10 + last_digit) as i64
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = args.get(1).unwrap();

    let input_file = open_input(filename).unwrap();

    match read_input(input_file) {
        Ok(inputs) => {
            let calibration_sum_1: i64 = inputs.iter().map(|l| get_calibration_value(l)).sum();
            println!("part 1 answer {:?}", calibration_sum_1);

            let calibration_sum_2: i64 = inputs.iter().map(|l| get_spelled_calibration_value(l)).sum();
            println!("part 2 answer {:?}", calibration_sum_2);

        }
        Err(err) => println!("could not parse input {:?}", err),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_calibration_value() {
        let inputs = vec![String::from("1abc2"), String::from("pqr3stu8vwx"), String::from("a1b2c3d4e5f"), String::from("treb7uchet")];

        assert_eq!(get_calibration_value(&inputs[0]), 12);
        assert_eq!(get_calibration_value(&inputs[1]), 38);
        assert_eq!(get_calibration_value(&inputs[2]), 15);
        assert_eq!(get_calibration_value(&inputs[3]), 77);
    }

    #[test]
    fn test_get_spelled_calibration_value() {
        let inputs = vec![String::from("two1nine"), String::from("eightwothree"), String::from("abcone2threexyz"), String::from("xtwone3four"),
        String::from("4nineeightseven2"), String::from("zoneight234"), String::from("7pqrstsixteen"), String::from("mztttgnxdqt4"),
        String::from("8threesevenfourgbgteight5twonenjr")];

        assert_eq!(get_spelled_calibration_value(&inputs[0]), 29);
        assert_eq!(get_spelled_calibration_value(&inputs[1]), 83);
        assert_eq!(get_spelled_calibration_value(&inputs[2]), 13);
        assert_eq!(get_spelled_calibration_value(&inputs[3]), 24);
        assert_eq!(get_spelled_calibration_value(&inputs[4]), 42);
        assert_eq!(get_spelled_calibration_value(&inputs[5]), 14);
        assert_eq!(get_spelled_calibration_value(&inputs[6]), 76);        
        assert_eq!(get_spelled_calibration_value(&inputs[7]), 44);        
        assert_eq!(get_spelled_calibration_value(&inputs[8]), 81);
    }
}
