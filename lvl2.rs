// use std::env;
// use std::fs::File;
// use std::io::{BufRead, BufReader, Write};

// fn main() {
//     let args: Vec<String> = env::args().collect();
//     let input_file_path = args[1].to_owned();
//     let output_file_path = format!("{}.out", input_file_path);

//     let input_file = File::open(input_file_path).expect("Error opening input file");
//     let reader = BufReader::new(input_file);

//     // skip the first two line
//     let mut reader = reader.lines();
//     reader.next();
//     reader.next();
//     reader.next();

//     let mut coins = Vec::new();
//     let mut amounts = Vec::new();

//     let mut output_file =
//         File::create(output_file_path.clone()).expect("Error creating output file");

//     // loop over other lines
//     for line in reader {
//         let line = line.expect("Error reading line");
//         coins.clear();
//         for coin in line.split_whitespace() {
//             // add coin parsed to int to coins vector
//             coins.push(coin.parse::<i32>().expect("Error parsing coin"));
//         }
//         // get next line of file in the input file to the line variable
//         let line = reader
//             .next()
//             .expect("Error reading line")
//             .expect("Error reading line");

//         for amount in line.split_whitespace() {
//             // add coin parsed to int to coins vector
//             amounts.push(amount.parse::<i32>().expect("Error parsing coin"));
//         }
//         for amount in amounts.clone() {
//             for coin in coins {
//                 if amount - coin > 0 && coins.contains(&(amount - coin)) {
//                     println!("{} {}", coin, amount - coin);
//                     break;
//                 }
//             }
//         }
//         // println!("{:?}", coins);
//     }
// }

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
    let mut reader = reader.lines().skip(2);

    let mut coins = Vec::new();
    let mut amounts = Vec::new();

    let mut output_file =
        File::create(output_file_path.clone()).expect("Error creating output file");

    // loop over other lines
    let iter = &mut reader;
    let amount_num = iter
        .next()
        .expect("Error reading line")
        .expect("Error reading line")
        .parse::<i32>()
        .expect("Error parsing amount");
    while let Some(line) = iter.next() {
        let line = line.expect("Error reading line");
        coins.clear();
        for coin in line.split_whitespace() {
            coins.push(coin.parse::<i32>().expect("Error parsing coin"));
        }
        // get next line of file in the input file to the line variable
        let line = iter
            .next()
            .expect("Error reading line")
            .expect("Error reading line");

        for amount in line.split_whitespace() {
            amounts.push(amount.parse::<i32>().expect("Error parsing amount"));
        }

        let mut temp_amount = amount_num;
        for amount in amounts.clone() {
            for coin in &coins {
                if amount - coin > 0 && coins.contains(&(amount - coin)) {
                    writeln!(output_file, "{} {}", coin, amount - coin);
                    temp_amount -= 1;
                    break;
                }
            }
            if temp_amount == 0 {
                break;
            }
        }
        // writeln!(output_file, "\n");
    }
}
