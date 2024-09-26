use std::fs::{File, OpenOptions};
use std::io::{self, Read, Write};

pub fn create_file(filename: &str) -> io::Result<()> {
    let mut file = File::create(filename)?;
    file.write_all(b"Hello, world!")?;
    Ok(())
}

pub fn read_file(filename: &str) -> io::Result<String> {
    let mut file = File::open(filename)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

pub fn append_file(filename: &str, text: &str) -> io::Result<()> {
    let mut file = OpenOptions::new()
        .append(true)
        .open(filename)?;

    writeln!(file, "{}", text)?;
    Ok(())
}

pub fn delete_file(filename: &str) -> io::Result<()> {
    std::fs::remove_file(filename)?;
    Ok(())
}
