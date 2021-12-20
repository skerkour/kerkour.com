use std::{
    fs::{self, File},
    io::{BufRead, BufReader, Read},
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    read_file_string("Cargo.toml")?;
    read_file_vec("Cargo.toml")?;
    read_file_line_by_line("Cargo.toml")?;
    read_file_buffer("Cargo.toml")?;

    Ok(())
}

fn read_file_string(filepath: &str) -> Result<String, Box<dyn std::error::Error>> {
    let data = fs::read_to_string(filepath)?;
    Ok(data)
}

fn read_file_vec(filepath: &str) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let data = fs::read(filepath)?;
    Ok(data)
}

fn read_file_line_by_line(filepath: &str) -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open(filepath)?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        println!("{}", line?);
    }

    Ok(())
}

fn read_file_buffer(filepath: &str) -> Result<(), Box<dyn std::error::Error>> {
    const BUFFER_LEN: usize = 512;
    let mut buffer = [0u8; 512];

    let mut file = File::open(filepath)?;

    loop {
        let read_count = file.read(&mut buffer)?;

        do_something(&buffer[..read_count]);

        if read_count != BUFFER_LEN {
            break;
        }
    }
    Ok(())
}

fn do_something(_data: &[u8]) {}
