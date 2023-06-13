use std::env;
use std::fs::{self, File};
use std::io::{self, BufRead, Write};
use std::path::PathBuf;
use chrono::Local;

const DELIMITER: &str = "☕";

fn main() {
    let args: Vec<String> = env::args().collect();

    // Prompt for arguments if not provided
    let log_directory = if args.len() > 1 {
        PathBuf::from(&args[1])
    } else {
        println!("Please enter the path to the log directory:");
        let mut input = String::new();
        io::stdin().read_line(&mut input).ok();
        PathBuf::from(input.trim())
    };

    let wordlist_path = if args.len() > 2 {
        PathBuf::from(&args[2])
    } else {
        println!("Please enter the path to the wordlist file or directory:");
        let mut input = String::new();
        io::stdin().read_line(&mut input).ok();
        PathBuf::from(input.trim())
    };

    // Process log files
    process_log_directory(&log_directory, &wordlist_path);

    println!("Jetsam complete.");
}

fn process_log_directory(log_directory: &PathBuf, wordlist_path: &PathBuf) {
    if log_directory.is_dir() {
        if let Ok(entries) = fs::read_dir(log_directory) {
            for entry in entries {
                if let Ok(entry) = entry {
                    let path = entry.path();
                    if path.is_dir() {
                        process_log_directory(&path, wordlist_path);
                    } else if path.extension().unwrap_or_default() == "txt" {
                        process_log_file(&path, wordlist_path);
                    }
                }
            }
        }
    }
}

fn process_log_file(log_file_path: &PathBuf, wordlist_path: &PathBuf) {
    let wordlist: Vec<String> = load_wordlist(wordlist_path);

    if let Ok(file) = File::open(log_file_path) {
        let reader = io::BufReader::new(file);
        let mut line_number = 0;

        for line in reader.lines() {
            line_number += 1;
            if let Ok(line_content) = line {
                let line_fields: Vec<&str> = line_content.split(DELIMITER).collect();
                if line_fields.len() >= 6 {
                    let sender = line_fields[4];
                    let message = line_fields[5];

                    if wordlist.iter().any(|word| message.contains(word)) {
                        // Sanitize the line by adding a # in column 1
                        let sanitized_line = format!("#{}{}", DELIMITER, line_content);

                        // Log the sanitized line to the Jetsam log
                        let timestamp = Local::now().format("%Y%m%d-%H%M%S").to_string();
                        let log_content = format!(
                            "Timestamp: {}\nPath: {}\nLine Number: {}\nOriginal Line: {}\n",
                            timestamp,
                            log_file_path.display(),
                            line_number,
                            line_content
                                .replace(DELIMITER, " ")
                                .replace(" ", "")
                        );
                        let log_filename = format!("jetsam{}.log", timestamp);
                        let log_path = dirs::home_dir()
                            .unwrap_or_else(|| PathBuf::from("."))
                            .join(&log_filename);

                        // Write the log content to the logfile
                        if let Ok(mut file) = File::create(&log_path) {
                            file.write_all(log_content.as_bytes()).ok();
                            println!("Jetsam log created at: {}", log_path.display());
                        } else {
                            println!("Failed to create Jetsam log at: {}", log_path.display());
                        }

                        // Modify the line in the log file
                        if let Ok(mut file) = fs::OpenOptions::new().write(true).open(log_file_path) {
                            let modified_line = format!("#{}{}", DELIMITER, sanitized_line);
                            let _ = file.write_fmt(format_args!(
                                "{}\n",
                                modified_line
                            ));
                        }
                    }
                }
            }
        }
    } else {
        println!("Failed to open log file: {}", log_file_path.display());
    }
}

fn load_wordlist(wordlist_path: &PathBuf) -> Vec<String> {
    let mut wordlist: Vec<String> = Vec::new();

    if wordlist_path.is_dir() {
        if let Ok(entries) = fs::read_dir(wordlist_path) {
            for entry in entries {
                if let Ok(entry) = entry {
                    let path = entry.path();
                    if path.is_file() {
                        let extension = path.extension().unwrap_or_default();
                        if extension == "txt" {
                            if let Ok(file) = File::open(&path) {
                                let reader = io::BufReader::new(file);
                                for line in reader.lines() {
                                    if let Ok(word) = line {
                                        wordlist.push(word.trim().to_string());
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    } else if wordlist_path.is_file() {
        if let Ok(file) = File::open(wordlist_path) {
            let reader = io::BufReader::new(file);
            for line in reader.lines() {
                if let Ok(word) = line {
                    wordlist.push(word.trim().to_string());
                }
            }
        }
    }

    wordlist
}
