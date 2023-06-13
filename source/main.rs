use std::env;
use std::fs::{self, File};
use std::io::{self, BufRead, Write};
use std::path::{Path, PathBuf};
use chrono::Local;

fn main() {
    // Get the command-line arguments
    let args: Vec<String> = env::args().collect();

    // Check if two arguments (log directory path and wordlist path) are provided
    if args.len() != 3 {
        println!("Please provide the log directory path and wordlist path.");
        return;
    }

    // Read the log directory path and wordlist path
    let log_directory_path = &args[1];
    let wordlist_path = &args[2];

    // Process the log files recursively
    process_log_directory(Path::new(log_directory_path), wordlist_path);

    println!("Jetsam operation completed.");
}

fn process_log_directory(log_directory_path: &Path, wordlist_path: &str) {
    // Iterate over the entries in the log directory
    if let Ok(entries) = fs::read_dir(log_directory_path) {
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();
                if path.is_dir() {
                    // Recursively process subdirectories
                    process_log_directory(&path, wordlist_path);
                } else if let Some(extension) = path.extension() {
                    if extension == "txt" {
                        // Process text log files
                        process_log_file(&path, wordlist_path);
                    }
                }
            }
        }
    }
}

fn process_log_file(log_file_path: &Path, wordlist_path: &str) {
    // Open the log file
    if let Ok(file) = File::open(log_file_path) {
        let reader = io::BufReader::new(file);

        // Read the lines of the log file
        for (line_number, line) in reader.lines().enumerate() {
            if let Ok(line_content) = line {
                if is_line_modified(&line_content, wordlist_path) {
                    // Modify the line by adding # in column 1
                    let modified_line = format!("#{}{}", &line_content[1..], '\n');

                    // Log the modification
                    log_modification(log_file_path, line_number + 1, &line_content, &modified_line);
                }
            }
        }
    }
}

fn is_line_modified(line: &str, wordlist_path: &str) -> bool {
    // Read the wordlist file(s) recursively
    let wordlist_path = Path::new(wordlist_path);

    if wordlist_path.is_file() {
        if let Ok(file) = File::open(wordlist_path) {
            let reader = io::BufReader::new(file);
            for word in reader.lines() {
                if let Ok(word) = word {
                    if line.contains(&word) {
                        return true;
                    }
                }
            }
        }
    } else if wordlist_path.is_dir() {
        if let Ok(entries) = fs::read_dir(wordlist_path) {
            for entry in entries {
                if let Ok(entry) = entry {
                    let path = entry.path();
                    if let Some(extension) = path.extension() {
                        if extension == "txt" {
                            if is_line_modified(line, path.to_str().unwrap()) {
                                return true;
                            }
                        }
                    }
                }
            }
        }
    }

    false
}

fn log_modification(log_file_path: &Path, line_number: usize, line_content: &str, modified_line: &str) {
    // Create the jetsam log file path
    let home_dir = match dirs::home_dir() {
        Some(path) => path,
        None => return,
    };

    let log_file_name = format!("jetsam{}.log", Local::now().format("%y%m%d-%H%M%S"));
    let jetsam_log_file_path = home_dir.join(log_file_name);

    // Open the jetsam log file in append mode
    let mut jetsam_log_file = match File::create(&jetsam_log_file_path) {
        Ok(file) => file,
        Err(_) => return,
    };

    // Write the modification details to the jetsam log file
    let log_message = format!(
        "{}\n{}\nLine {}\n{}\n{}\n\n",
        Local::now().format("%Y%m%d-%H%M%S"),
        log_file_path.display(),
        line_number,
        line_content.replace("☕", " "),
        modified_line.replace("☕", " ")
    );

    if let Err(_) = jetsam_log_file.write_all(log_message.as_bytes()) {
        return;
    }
}
