use std::io::prelude::*;
use std::io;
use std::fs::File;
use std::io::BufReader;

use itertools::Itertools;

fn read_file_to_vector() -> Result<Vec<i32>, io::Error> {
    let report = File::open("report.txt")?;

    let mut reader = BufReader::new(report);
    let mut line = String::new();

    let mut expenses = Vec::new();

    loop {
        match reader.read_line(&mut line) {
            Ok(bytes_read) => {
                if bytes_read == 0 {
                    break;
                }
                let line_as_int: i32 = line.trim().parse::<i32>().unwrap();
                expenses.push(line_as_int);
                line.clear();
            },
            Err(err) => {
                println!("Error reading file: {}", err);
            }
        };
    }

    Ok(expenses)
}

fn k_combination_n_sum(expenses: &Vec<i32>, k: usize, n: i32) -> Option<Vec<i32>> {
    expenses
        .iter()
        .copied()
        .combinations(k)
        .find(|combination| combination.iter().sum::<i32>() == n)
}

// Find entries in "report.txt" that sum to 2020, and print their product
fn main() -> std::io::Result<()> {

    let expenses = read_file_to_vector()?;

    let combinations = k_combination_n_sum(&expenses, 3, 2020);

    let product: i32 = combinations.unwrap().iter().product();

    println!("Product of combination: {}", product);
    Ok(())
}
