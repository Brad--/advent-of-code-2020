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

// returns (min, max, char, password)
fn parse_rule(password_with_rule: String) -> (usize, usize, String, String) {
    let mut pass_parts = password_with_rule.split_whitespace();

    let rule = pass_parts.next().unwrap();

    let (rule_min, rule_max) = rule.split_at(rule.find('-').unwrap());
    let min = rule_min.parse::<usize>().unwrap();
    let max = rule_max.replace("-", "").parse::<usize>().unwrap();

    let rule_char = pass_parts.next().unwrap().replace(":", "");
    let password = pass_parts.next().unwrap();

    (min, max, rule_char, password.to_string())
}

fn validate_password_with_count_rule(password_with_rule: String) -> bool {
    let (min, max, rule_char, password) = parse_rule(password_with_rule);

    let match_count = password.matches(&rule_char).count();

    match_count >= min && match_count <= max
}

fn validate_password_with_pos_rule(password_with_rule: String) -> bool {
    let (mut first_pos, mut second_pos, rule_char, password) = parse_rule(password_with_rule);
    first_pos -= 1;
    second_pos -= 1;
    password
        .match_indices(&rule_char)
        .collect::<Vec<_>>()
        .iter()
        .filter(|(pos, _m)| *pos == first_pos || *pos == second_pos)
        .count() == 1
}

fn main() -> Result<(), io::Error> {
    let passwords: Vec<String> = read_file_to_vector("passwords.txt")?;

    let valid_count = passwords.iter()
        .filter(|pass| validate_password_with_pos_rule(pass.to_string()))
        .count();

    println!("Found {} valid passwords", valid_count);

    Ok(())
}