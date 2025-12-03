use std::fs;
use substring::Substring;

fn main() {
    // Load the input file
    // TODO: Handle both cases with a command line argument
    let file_path = String::from("test.dat");
    // let file_path = String::from("input.dat");
    println!("Reading input file '{file_path}'");
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
                println!("{}", bank);
                let mut i = 0;
                let length = bank.len();
                for battery in bank.split("") {
                    match battery {
                        "" => {}
                        _ => {
                            i = i + 1;  // TODO: Neater enumeration
                            for other in bank.substring(i, length).chars() {
                                // TODO: Neater concatenation
                                let concat = String::from(battery) + &String::from(other);

                                // Convert the concatenated value to an integer
                                let joltage: i32 = match concat.trim().parse() {
                                    Ok(num) => num,
                                    Err(_) => continue, // TODO: Error handling
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
