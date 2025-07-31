use std::fs::File;
use std::io::{BufReader, BufRead};

pub fn test_log() -> Result<(), Box<dyn std::error::Error>> {
    let path = "/var/log/syslog";
    let file = File::open(path)?;
    let reader = BufReader::new(file);

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

fn process_line(str: String) {
    println!("---process_line {}", str);
}

// fn handle_error(std::io::Error err) {
//     println!("---handle_error {}", err);
// }