/* use std::env;
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
} */

use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};

fn main() {
    let args: Vec<String> = env::args().collect();
    let input_file_path = &args[1];
    let output_file_path = format!("{}.out", input_file_path);

    let input_file = File::open(input_file_path).expect("Error opening input file");
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

        // Vector to store the fewest number of coins needed for each amount from 1 to 100
        let mut fewest_coins = vec![1000; 101]; // Initializing with a large number

        // Base case: no coins needed for amount 0
        fewest_coins[0] = 0;

        // Function to calculate the fewest number of coins needed for each amount
        for amount in 1..=100 {
            for coin in &coins {
                if *coin <= amount {
                    fewest_coins[amount as usize] = fewest_coins[amount as usize]
                        .min(fewest_coins[(amount - coin) as usize] + 1);
                }
            }
        }

        // Writing the fewest number of coins needed for each amount to the output file
        for amount in 1..=100 {
            let mut remaining_amount = amount;
            while remaining_amount > 0 {
                for &coin in coins.iter().rev() {
                    if remaining_amount - coin >= 0
                        && fewest_coins[remaining_amount as usize]
                            == fewest_coins[(remaining_amount - coin) as usize] + 1
                    {
                        write!(output_file, "{}x{}", remaining_amount / coin, coin)
                            .expect("Error writing to file");
                        remaining_amount -= coin * (remaining_amount / coin);
                        if remaining_amount > 0 {
                            write!(output_file, " ").expect("Error writing to file");
                        }
                        break;
                    }
                }
            }
            writeln!(output_file, "").expect("Error writing to file");
        }
    }
}

