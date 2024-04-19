use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};

fn main() {
    let args: Vec<String> = env::args().collect();
    let input_file_path = args[1].to_owned();
    let output_file_path = format!("{}.out", input_file_path);

    let input_file = File::open(&input_file_path).expect("Error opening input file");
    let reader = BufReader::new(input_file);

    // skip the first two lines
    let mut iter = reader.lines().skip(2);

    let mut output_file =
        File::create(output_file_path.clone()).expect("Error creating output file");

    while let Some(line) = iter.next() {
        let line = line.expect("Error reading line");
        let coins: Vec<i32> = line
            .split_whitespace()
            .map(|s| s.parse::<i32>().expect("Error parsing coin"))
            .collect();
        // Function to recursively find the fewest number of coins needed for an amount

        // Call backtrack for each amount from 1 to 100
        for mut amount in 1..=100 {
            // loop over coins from back to front
            for &coin in coins.iter().rev() {
                let mut counter = 0;
                while amount - coin >= 0 && amount != 0 && amount >= coin {
                    amount -= coin;
                    counter += 1;
                }
                if counter >= 1 {
                    write!(output_file, "{}x{}", counter, coin).expect("Error writing to file");
                    if amount != 0 {
                        write!(output_file, " ").expect("Error writing to file");
                    }
                }
                if (amount) == 0 {
                    writeln!(output_file, "").expect("Error writing to file");
                    break;
                }
            }
        }
    }
}
