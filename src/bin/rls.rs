extern crate chrono;

use chrono::{DateTime, TimeZone, Utc};
use std::time::SystemTime;
use std::{env, fs};

struct File {
    filename: String,
    is_dir: bool,
    modified_time: Option<DateTime<Utc>>,
    size: Option<u64>,
}

impl Default for File {
    fn default() -> File {
        File {
            filename: String::new(),
            is_dir: false,
            modified_time: None,
            size: None,
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let long_format = args.iter().any(|x| x == "-l");

    let dir = env::current_dir().unwrap();

    println!("{}", dir.display());

    for entry in fs::read_dir(dir).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();

        let metadata = fs::metadata(&path).unwrap();

        let filename = match path.file_name() {
            Some(name) => name.to_str().unwrap().to_string(),
            None => "No filename".to_string(),
        };

        let mut file = File {
            filename: filename,
            is_dir: metadata.is_dir(),
            ..Default::default()
        };

        if long_format {
            let modified_time = metadata
                .modified()
                .unwrap_or(SystemTime::now())
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs() as i64;
            file.modified_time = Some(Utc.timestamp(modified_time, 0));

            file.size = Some(metadata.len());

            println!(
                "{}{}\t{} bytes\tLast modified: {}",
                if file.is_dir { "./" } else { "" },
                file.filename,
                file.size.unwrap(),
                file.modified_time.unwrap().format("%b %e %Y %k:%M")
            );
        } else {
            print!("{}{}\t", if file.is_dir { "./" } else { "" }, file.filename);
        }
    }

    // extra new line at the end
    if !long_format {
        println!("");
    }
}
