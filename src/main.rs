use std::collections::HashSet;
use std::env;
use std::fs;
use std::io::{self, BufRead, BufReader};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::thread;
use colored::*;

fn search_file(file_path: &Path, pattern: &str) -> io::Result<Vec<(usize, String)>> {
    let file = fs::File::open(file_path)?;
    let reader = BufReader::new(file);
    let mut matches = Vec::new();
    let mut matched_lines = HashSet::new();

    for (i, line) in reader.lines().enumerate() {
        let line = match line {
            Ok(line) => line,
            Err(_) => continue,
        };
        if line.contains(pattern) {
            let line_matches = line
                .match_indices(pattern)
                .map(|(start, _)| start)
                .collect::<Vec<_>>();
            for _ in line_matches {
                let line_number = i + 1;
                if matched_lines.contains(&line_number) {
                    continue;
                }
                let formatted_line = format!(
                    "{}:{}\n\t{}",
                    file_path.display(),
                    line_number,
                    &line
                );
                matched_lines.insert(line_number);
                matches.push((line_number, formatted_line));
            }
        }
    }
    Ok(matches)
}

fn search_directory(
    path: &Path,
    pattern: &str,
    matches: Arc<Mutex<Vec<(usize, String)>>>,
) -> io::Result<()> {
    if path.is_file() {
        let file_matches = search_file(path, pattern)?;
        let mut matches = matches.lock().unwrap();
        matches.extend(file_matches);
    } else if path.is_dir() {
        let mut threads = Vec::new();
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let entry_path = entry.path();
            if entry_path.is_file() && entry_path.file_name().unwrap().to_str().unwrap().contains(pattern) {
                let file_matches = search_file(&entry_path, pattern)?;
                let mut matches = matches.lock().unwrap();
                matches.extend(file_matches);
            } else if entry_path.is_dir() && entry_path.file_name().unwrap().to_str().unwrap().contains(pattern) {
                let matches = matches.clone();
                let pattern = pattern.to_owned();
                let thread = thread::spawn(move || {
                    search_directory(&entry_path, &pattern, matches).unwrap();
                });
                threads.push(thread);
            } else {
                let matches = matches.clone();
                let pattern = pattern.to_owned();
                let thread = thread::spawn(move || {
                    search_directory(&entry_path, &pattern, matches).unwrap();
                });
                threads.push(thread);
            }
        }
        for thread in threads {
            thread.join().unwrap();
        }
    }
    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: {} pattern [path]", args[0]);
        return;
    }

    let pattern = &args[1];

    let path = if args.len() > 2 {
        PathBuf::from(&args[2])
    } else {
        env::current_dir().unwrap()
    };

    let matches = Arc::new(Mutex::new(Vec::new()));

    search_directory(&path, pattern, matches.clone()).unwrap();

    let matches = matches.lock().unwrap().clone();

    for (line_number, line) in &matches {
        let output = line.replace(pattern, &pattern.red().to_string());
        println!("{}: {}",line_number, output);
    }
    println!("Total matches: {}", matches.len());
}
