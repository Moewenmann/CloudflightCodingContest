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

    let mut coins = Vec::new();

    let mut output_file =
        File::create(output_file_path.clone()).expect("Error creating output file");

    while let Some(line) = iter.next() {
        let line = line.expect("Error reading line");
        coins.clear();
        for coin in line.split_whitespace() {
            coins.push(coin.parse::<i32>().expect("Error parsing coin"));
        }

        // Vector to store the fewest number of coins needed for each amount from 1 to 100
        let mut fewest_coins = vec![1000; 101]; // Initializing with a large number

        // Base case: no coins needed for amount 0
        fewest_coins[0] = 0;

        for amount in 1..=100 {
            // For each coin, check if it can be used to make up the current amount
            for coin in &coins {
                if *coin <= amount {
                    // If using this coin reduces the number of coins needed for the current amount
                    // update the fewest_coins array
                    fewest_coins[amount as usize] = fewest_coins[amount as usize]
                        .min(fewest_coins[(amount - coin) as usize] + 1);
                }
            }
        }

        // Writing the fewest number of coins needed for each amount to the output file
        writeln!(
            output_file,
            "{}",
            fewest_coins
                .iter()
                .map(|&x| x.to_string())
                .collect::<Vec<String>>()
                .join(" ")
        )
        .expect("Error writing to file");
    }
}
