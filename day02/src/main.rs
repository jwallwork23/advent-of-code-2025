use std::fs;
use substring::Substring;

fn main() {
    // Load the input file
    // TODO: Handle both cases with a command line argument
    // let file_path = String::from("test.dat");
    let file_path = String::from("input.dat");
    println!("Reading input file '{file_path}'");
    let contents = fs::read_to_string(file_path).expect("Failed to read input file");

    // Define solution variables as mutable integers
    let mut part1: i64 = 0;
    let mut part2: i64 = 0;

    // Loop over codes in the input file
    for entry in contents.split(",") {
        match entry {
            "" => {} // Skip any empty strings
            _ => match String::from(entry).find("-") {
                Some(num) => {
                    // Convert the start value to an integer
                    let start: i64 = match entry.substring(0, num).trim().parse() {
                        Ok(num) => num,
                        Err(_) => continue, // TODO: Error handling
                    };

                    // Convert the end value to an integer
                    let end: i64 = match entry.substring(num + 1, entry.len()).trim().parse() {
                        Ok(num) => num,
                        Err(_) => continue, // TODO: Error handling
                    };

                    // Loop over the range
                    for code in start..end + 1 {
                        // Convert back to a String
                        let code_str: String = code.to_string();
                        let length = code_str.len();

                        // Loop over all possible divisors
                        'divisor_loop: for divisor in 2..length + 1 {
                            // Check that the length divides by the divisor
                            let ratio = length / divisor;
                            if ratio * divisor != length {
                                continue;
                            }

                            // Compare each substring against the first
                            let first = code_str.substring(0, ratio);
                            for i in 1..divisor {
                                if code_str.substring(i * ratio, (i + 1) * ratio) != first {
                                    continue 'divisor_loop;
                                }
                            }

                            // If all substrings match then we have an 'invalid code'
                            if divisor == 2 {
                                part1 = part1 + code;
                            }
                            part2 = part2 + code;

                            // Drop the code once it's been deemed invalid
                            break;
                        }
                    }
                }
                None => {}
            },
        }
    }

    println!("part 1: {}", part1);
    println!("part 2: {}", part2)
}
