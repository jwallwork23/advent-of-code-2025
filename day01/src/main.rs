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

    // Process the contents as needed

    // Initialise dial position to 50
    let mut pos: i32 = 50;

    // Define solution variables as mutable integers
    let mut count: i32 = 0;  // Number of times the dial moves to zero
    let mut clicks: i32 = 0; // Number of times the dial moves past zero

    // Loop over codes in the input file
    for entry in contents.split("\n") {
        match entry {
            "" => {} // Skip any empty strings
            _ => {
                // Deduce the direction
                let direction: i32 = match entry.substring(0, 1) {
                    "L" => -1,
                    "R" => 1,
                    _ => {
                        eprintln!("WARNING Failed to parse direction");
                        continue
                    }
                };

                // Deduce the magnitude
                let mut magnitude: i32 = match entry.substring(1, entry.len()).trim().parse() {
                    Ok(num) => num,
                    Err(e) => {
                        eprintln!("WARNING Failed to parse magnitude: {}", e);
                        continue
                    }
                };

                // Adjust magnitude modulo 100
                while magnitude > 99 {
                    magnitude -= 100;
                    if magnitude != 0 {
                        clicks += 1;
                    }
                }
                if magnitude == 0 {
                    continue;
                }

                // Move the dial according to the direction and magnitude
                let prev: i32 = pos;
                pos += direction * magnitude;

                // Adjust modulo 100
                if pos < 0 || pos > 99 {
                    if pos < 0 {
                        pos += 100;
                    } else {
                        pos -= 100;
                    }
                    // Count clicks *past* zero
                    if prev != 0 && pos != 0 {
                        clicks += 1;
                    }
                }

                // Count the number of times the dial position *lands on* zero
                if pos == 0 {
                    count += 1;
                    clicks += 1;
                }
            }
        };
    }

    // Report the results for each part of the problem
    println!("part 1: password={count}");
    println!("part 2: clicks={clicks}");
}
