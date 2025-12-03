use std::env;
use std::fs;
use substring::Substring;

fn main() {
    // Load the input file
    let args: Vec<String> = env::args().collect();
    let file_path = if args.len() > 1 {
        args[1].clone()
    } else {
        String::from("input.dat")
    };
    println!("Reading input file '{}'", file_path);
    let contents = fs::read_to_string(file_path).expect("Failed to read input file");

    // Define solution variables as mutable integers
    let mut part1: i32 = 0;
    let mut part2: i32 = 0;

    // Loop over codes in the input file
    for bank in contents.split("\n") {
        let mut max: i32 = 0;
        match bank {
            "" => {} // Skip any empty strings
            _ => {
                // println!("{}", bank);
                let length = bank.len();
                for (i, battery) in bank.split("").enumerate() {
                    match battery {
                        "" => {}
                        _ => {
                            for other in bank.substring(i, length).chars() {
                                // Convert the concatenated value to an integer
                                let joltage: i32 =
                                    match format!("{}{}", battery, other).trim().parse() {
                                        Ok(num) => num,
                                        Err(e) => {
                                            eprintln!("WARNING Failed to parse joltage: {}", e);
                                            continue;
                                        }
                                    };

                                if joltage > max {
                                    max = joltage;
                                }
                            }
                        }
                    }
                }
            }
        }
        part1 = part1 + max;
    }

    println!("part 1: {}", part1);
    println!("part 2: {}", part2)
}
