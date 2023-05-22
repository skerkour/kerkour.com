+++
date = 2021-12-09T06:00:00Z
title = "4 ways to read a file in Rust"
type = "post"
tags = ["programming", "rust", "tutorial"]
authors = ["Sylvain Kerkour"]
url = "/rust-read-file"

[extra]
lang = "en"

comment ="""
"""
+++


## Read the entire file as a String

```rust
fn read_file_string(filepath: &str) -> Result<String, Box<dyn std::error::Error>> {
    let data = fs::read_to_string(filepath)?;
    Ok(data)
}
```

## Read the entire file as a Vector


```rust
fn read_file_vec(filepath: &str) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let data = fs::read(filepath)?;
    Ok(data)
}
```

## Read a text file line by line

```rust
fn read_file_line_by_line(filepath: &str) -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open(filepath)?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        println!("{}", line?);
    }

    Ok(())
}
```



## Read a file with a buffer

```rust
fn read_file_buffer(filepath: &str) -> Result<(), Box<dyn std::error::Error>> {
    const BUFFER_LEN: usize = 512;
    let mut buffer = [0u8; BUFFER_LEN];
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
```

Alternatively, you can use [BufReader](https://doc.rust-lang.org/std/io/struct.BufReader.html).


## The code is on GitHub

As always, the code is on GitHub: [github.com/skerkour/kerkour.com](https://github.com/skerkour/kerkour.com/tree/main/blog/2021/rust_file_encryption) (please don't forget to star the repo 🙏).
