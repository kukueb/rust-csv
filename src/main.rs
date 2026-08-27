mod errors;
mod orders;

use errors::AppError;
use orders::Order;

// use std::any::type_name_of_val;
use std::env;
use std::fs::{self, File};
use std::io::{self, BufRead, BufReader};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Not enough arguments!");
        return;
    }

    let data = read_lines(args[1].clone());

    let data_value;

    match data {
        Ok(val) => {
            println!("Data successfully extracted from file!");
            data_value = val
        }
        Err(e) => panic!("An error occured on data extraction: {}", e),
    }

    // println!("{}", data_value[0]);
    // println!("{}", data_value[1]);

    let split_0 = data_value[0].split(',');
    let split_1 = data_value[1].split(',');

    let max_key: Option<&str> = split_0.clone().max_by_key(|v| v.len());
    let max_key_len: usize = max_key.map(|v| v.len()).unwrap_or(0);

    let combined: Vec<[&str; 2]> = split_0.zip(split_1).map(|(k, v)| [k, v]).collect();

    for [k, v] in &combined {
        let formatted_key = " ".repeat(max_key_len - k.len()) + k;
        println!("{} <=> {}", formatted_key, v);
    }

    // println!("{}", combined[4][1]);
    // println!("{}", stamp.format("%Y-%m-%d %H:%M:%S").to_string());

    let order_1 = Order::from_string(&data_value[1]);

    println!("----------------- order_1 -----------------");
    match order_1 {
        Ok(value) => println!("{}", value),
        Err(err) => eprintln!("Error on display: {}", err),
    }
}

fn is_valid_file_path(file_path: &str) -> bool {
    let file: Result<bool, io::Error> = fs::exists(file_path);

    match file {
        Ok(value) => return value,
        Err(error) => panic!(
            "There's an error occured on file existance check: {}",
            error
        ),
    }
}

fn parse_lines_as_struct(data: Vec<String>) {}

fn read_lines(file_path: String) -> Result<Vec<String>, AppError> {
    if !is_valid_file_path(&file_path) {
        return Err(AppError::FileReadingError(file_path));
    }

    let file: File = File::open(file_path).unwrap_or_else(|e| panic!("Error in read_lines: {}", e));

    let reader = BufReader::new(file);
    let mut data: Vec<String> = Vec::<String>::new();

    for line in reader.lines() {
        match line {
            Ok(val) => {
                data.push(val.clone());
            }
            Err(e) => {
                println!("Line read with an error: {} [ABORTING]", e);
                break;
            }
        }
    }

    return Ok(data);
}
