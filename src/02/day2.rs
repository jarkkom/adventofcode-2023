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

fn is_game_possible<'a, I>(sets: I) -> bool 
where
    I: Iterator<Item = &'a str>
{
    const MAX_RED: i32 = 12;
    const MAX_GREEN: i32 = 13;
    const MAX_BLUE: i32 = 14;

    for set in sets {
        let mut counts = HashMap::new();
        for color_count in set.split(", ") {
            let parts: Vec<&str> = color_count.split_whitespace().collect();
            let count: i32 = parts[0].parse().unwrap();
            let color = parts[1];

            *counts.entry(color).or_insert(0) += count;
        }

        if *counts.get("red").unwrap_or(&0) > MAX_RED
            || *counts.get("green").unwrap_or(&0) > MAX_GREEN
            || *counts.get("blue").unwrap_or(&0) > MAX_BLUE
        {
            return false;
        }
    }

    true
}

fn find_minimum_cubes<'a, I>(sets: I) -> HashMap<&'a str, i32> 
where
    I: Iterator<Item = &'a str>
{
    let mut min_cubes: HashMap<&str, i32> = HashMap::new();

    for set in sets {
        for color_count in set.split(", ") {
            let parts: Vec<&str> = color_count.split_whitespace().collect();
            let count: i32 = parts[0].parse().unwrap_or(0);
            let color = parts[1];

            *min_cubes.entry(color).or_insert(0) = i32::max(*min_cubes.get(color).unwrap_or(&0), count);
        }
    }

    min_cubes
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = args.get(1).unwrap();

    let input_file = open_input(filename).unwrap();

    match read_input(input_file) {
        Ok(inputs) => {
            let mut sum_of_possible_games = 0;
            let mut total_power_sum = 0;
            for line in inputs {
                let parts: Vec<&str> = line.trim().split(": ").collect();
                let game_id: i32 = parts[0].split_whitespace().nth(1).unwrap().parse().unwrap();
                let sets = parts[1].split("; ");
        
                if is_game_possible(sets.clone()) {
                    sum_of_possible_games += game_id;
                }

                let min_cubes = find_minimum_cubes(sets);
                let power = min_cubes.values().product::<i32>();
                total_power_sum += power;
            }
        
            println!("Sum of possible game IDs: {}", sum_of_possible_games);
            println!("Total sum of the powers: {}", total_power_sum);
        }
        Err(err) => println!("could not parse input {:?}", err),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
 
}
