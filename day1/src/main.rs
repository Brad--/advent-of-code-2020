use std::io::prelude::*;
use std::fs::File;
use std::io::BufReader;

fn main() -> std::io::Result<()> {
    let report = File::open("report.txt")?;

    let mut reader = BufReader::new(report);
    let mut line = String::new();

    loop {
        match reader.read_line(&mut line) {
            Ok(bytes_read) => {
                if bytes_read == 0 {
                    break;
                }
                print!("read: {}", line);
                line.clear();
            },
            Err(err) => {
                println!("Error reading file: {}", err);
            }
        };
    }
    
    Ok(())
}
