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

//     let mut coins = Vec::new();

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
//         for value in 1..1000 {
//             if !coins.contains(&value) {
//                 writeln!(output_file, "{}", value).expect("Error writing to file");
//                 break;
//             }
//         }
//         // println!("{:?}", coins);
//     }
// }
