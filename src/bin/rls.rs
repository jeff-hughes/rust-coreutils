extern crate chrono;
use std::{env, fs};
use std::time::SystemTime;
use chrono::{Utc, TimeZone};

fn main() {
    let dir = env::current_dir().unwrap();

    println!("{}", dir.display());

    for entry in fs::read_dir(dir).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();

        let metadata = fs::metadata(&path).unwrap();

        let modified_time = metadata.modified().unwrap_or(SystemTime::now())
            .duration_since(std::time::UNIX_EPOCH).unwrap()
            .as_secs() as i64;
        let last_modified = Utc.timestamp(modified_time, 0).format("%Y-%m-%d %H:%M:%S");

        let mut filename = match path.file_name() {
            Some(name) => name.to_str().unwrap().to_string(),
            None => "No filename".to_string()
        };

        if metadata.is_dir() {
            filename = "./".to_string() + &filename;
        }

        println!(
            "{}\t{} bytes\tLast modified: {}",
            filename,
            metadata.len(),
            last_modified
        );
    }
}
