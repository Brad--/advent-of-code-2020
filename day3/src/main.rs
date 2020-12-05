use std::io::prelude::*;
use std::io;
use std::fs::File;
use std::io::BufReader;

fn read_file_to_vector(filename: &str) -> Result<Vec<String>, io::Error> {
    let report = File::open(filename)?;

    let mut reader = BufReader::new(report);
    let mut line = String::new();

    let mut vec: Vec<String> = Vec::new();

    loop {
        match reader.read_line(&mut line) {
            Ok(bytes_read) => {
                if bytes_read == 0 {
                    break;
                }
                vec.push(line.trim().to_string());
                line.clear();
            },
            Err(err) => {
                println!("Error reading file: {}", err);
            }
        };
    }

    Ok(vec)
}

fn traverse_slope(tree_map: &Vec<String>, x: usize, y: usize) -> i64 {
    let (mut hits, mut x_pos, mut y_pos) = (0, 0, 0);
    while y_pos < tree_map.len() {
        let trees: Vec<char> = tree_map[y_pos].chars().collect();
        if trees[x_pos % trees.len()] == '#' {
            hits += 1;
        }
        y_pos += y;
        x_pos += x;
    }

    hits
}

fn main() -> Result<(), io::Error> {
    let tree_map = read_file_to_vector("trees.txt")?;

    let slopes = [
        (1, 1),
        (3, 1),
        (5, 1),
        (7, 1),
        (1, 2)
    ];

    let hits: i64 = slopes.iter()
                .map(|(x, y)| traverse_slope(&tree_map, *x, *y))
                .product();

    println!("Hit {} trees", hits);
    Ok(())
}
