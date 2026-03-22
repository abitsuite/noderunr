// src/utils/logger.rs

use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn test_log() -> Result<(), Box<dyn std::error::Error>> {
    let path = "/var/log/syslog";
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    read_log(reader)
}

/**
 * Read Log
 *
 * Reads lines from any BufRead source and processes them.
 */
pub fn read_log<R: BufRead>(reader: R) -> Result<(), Box<dyn std::error::Error>> {
    for line in reader.lines() {
        match line {
            // line is a String
            Ok(line) => process_line(line),
            Err(_) => todo!(),
            // Err(err) => handle_error(err),
        }
    }

    Ok(())
}

pub(crate) fn process_line(str: String) {
    println!("---process_line {}", str);
}

// fn handle_error(std::io::Error err) {
//     println!("---handle_error {}", err);
// }
