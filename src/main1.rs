use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn analyze_file(path: &str) {
    let input = File::open(path).expect("Failed to open file");
    let reader = io::BufReader::new(input);

    for (line_number, line) in reader.lines().enumerate() {
        if let Ok(content) = line {
            if content.contains("unwrap()") {
                println!("[WARNING] unwrap() at line {}: {}", line_number + 1, content.trim());
            }
            if content.contains("todo!") {
                println!("[TODO] todo!() at line {}: {}", line_number + 1, content.trim());
            }
            if content.contains("unsafe") {
                println!("[DANGER] unsafe block at line {}: {}", line_number + 1, content.trim());
            }
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: rust-analyzer-tool <path_to_rust_file>");
        return;
    }

    let file_path = &args[1];
    if Path::new(file_path).exists() {
        analyze_file(file_path);
    } else {
        println!("File not found: {}", file_path);
    }
}
