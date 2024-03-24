use std::fs::File;
use std::io::{self, Read};

pub fn read_file(file_path: &str) -> String {
    let mut file = File::open(file_path).expect("File not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Failed to read file");
    contents
}

pub fn read_stdin() -> String {
    let mut contents = String::new();
    io::stdin().read_to_string(&mut contents).expect("Failed to read stdin");
    contents
}

pub fn report(message: String, location: String, line: i32) {
    eprintln!("| line {} | error {} : {}", line, location, message)
}

pub fn error(message: String, line: i32) {
    report(message, "".to_string(), line);
}
