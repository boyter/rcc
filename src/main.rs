extern crate walkdir;

use walkdir::WalkDir;
use std::fs::File;
use std::io::Read;
use std::io;
use walkdir::DirEntry;

fn main() -> Result<(), io::Error> {
    for entry in WalkDir::new("./").into_iter().filter_map(|e| e.ok()) {
        if !entry.file_type().is_file() {
            continue
        }

        let mut file_job = load_file(&entry)?;
        process_file(&mut file_job);

        println!("{} lines={} bytes={}", file_job.name, file_job.lines, file_job.bytes);
    }
    Ok(())
}

const NUL: u8 = 0;
const NEWLINE: u8 = 10;

fn load_file(entry: &DirEntry) -> Result<(FileJob), io::Error> {
    let path = entry.path();
    let mut file = File::open(path)?;
    let mut buffer = Vec::with_capacity(file.metadata().unwrap().len() as usize);

    file.read_to_end(&mut buffer)?;

    return Ok(FileJob {
        name: entry.path().display().to_string(),
        bytes: 0,
        blank: 0,
        code: 0,
        comment: 0,
        lines: 0,
        content: buffer
    })
}

fn process_file(file_job: &mut FileJob) -> Result<(), io::Error> {
    let mut bytes_count: u32;

    bytes_count = 0;

    for i in file_job.content.iter() {
        if i == &NUL {
            return Ok(())
        }

        if i == &NEWLINE {
            file_job.lines += 1
        }

        file_job.bytes += 1;
    }

    Ok(())
}

struct FileJob {
    name: String,
    lines: u32,
    code: u32,
    comment: u32,
    blank: u32,
    bytes: u32,
    content: Vec<u8>
}
