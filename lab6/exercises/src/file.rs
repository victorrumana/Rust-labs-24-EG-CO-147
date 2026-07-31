use std::fs::{self, File};
use std::io::{self, BufRead, BufReader, Write};
use std::path::Path;

pub fn write_log(path: &str, entries: &[&str]) -> io::Result<()> {
    let mut file = File::create(path)?;
    for entry in entries {
        writeln!(file, "{}", entry)?;
    }
    Ok(())
}

pub fn count_lines(path: &str) -> io::Result<usize> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    Ok(reader.lines().count())
}

// TODO 4: Recursively list all .rs files under a given directory
pub fn list_rs_files(dir: &Path) -> io::Result<Vec<String>> {
    let mut results = Vec::new();
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                let mut sub_files = list_rs_files(&path)?;
                results.append(&mut sub_files);
            } else if let Some(ext) = path.extension() {
                if ext == "rs" {
                    results.push(path.to_string_lossy().into_owned());
                }
            }
        }
    }
    Ok(results)
}

pub fn run() -> io::Result<()> {
    println!("--- Exercise D: File I/O ---");
    let path = "output.log";
    let entries = vec![
        "INFO Server started",
        "WARN High memory usage",
        "ERROR Disk full",
        "INFO Backup complete",
    ];

    write_log(path, &entries)?;

    let n = count_lines(path)?;
    println!("Wrote {} lines to {}", n, path);

    let content = fs::read_to_string(path)?;
    let errors: Vec<&str> = content
        .lines()
        .filter(|l| l.starts_with("ERROR"))
        .collect();
    println!("Error lines: {:?}", errors);

    fs::remove_file(path)?;

    println!("Found .rs files in 'src' (TODO 4): {:?}", list_rs_files(Path::new("src"))?);

    Ok(())
}