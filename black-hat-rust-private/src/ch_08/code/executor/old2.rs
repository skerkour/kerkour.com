use mmap::*;
use std::{env, fs, mem, ptr};

type Error = Box<dyn std::error::Error>;

fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Usage:");
        println!("executor [shellcode path]");
        return Ok(());
    }

    let shellcode = fs::read(&args[1])?;

    let opts = [
        MapOption::MapReadable,
        MapOption::MapWritable,
        MapOption::MapExecutable,
    ];

    let mapping = MemoryMap::new(shellcode.len(), &opts).unwrap();

    unsafe {
        ptr::copy(shellcode.as_ptr(), mapping.data(), shellcode.len());
        mem::transmute::<_, fn()>(mapping.data())();
    }

    Ok(())
}
