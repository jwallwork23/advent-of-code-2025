use std::env;
use std::fs;

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

    // Create a padded matrix to hold the input data
    let n = contents.split("\n").into_iter().count();
    let mut matrix = vec![vec![0; n + 2]; n + 2];

    // Loop over codes in the input file
    for (i, entry) in contents.split("\n").enumerate() {
        match entry {
            "" => {} // Skip any empty strings
            _ => {
                for (j, c) in entry.chars().enumerate() {
                    // Set entry of array to 1 if @ and 0 if .
                    match c {
                        '@' => matrix[i + 1][j + 1] = 1,
                        '.' => matrix[i + 1][j + 1] = 0,
                        _ => panic!("Unexpected character in input"),
                    }
                }
            }
        }
    }

    // Count the number of adjacent @ symbols to each @ position
    let stencil: Vec<i32> = vec![-1, 0, 1];
    for i in 1..n {
        for j in 1..n {
            if matrix[i][j] == 0 {
                continue;
            }
            let mut sum = 0;
            for &id in &stencil {
                let ii = (i as i32 + id) as usize;
                for &jd in &stencil {
                    let jj = (j as i32 + jd) as usize;
                    if (id == 0) && (jd == 0) {
                        continue;
                    }
                    sum += matrix[ii][jj];
                }
            }
            if sum < 4 {
                part1 += 1;
            }
        }
    }

    println!("part 1: {}", part1);
    println!("part 2: {}", part2)
}
