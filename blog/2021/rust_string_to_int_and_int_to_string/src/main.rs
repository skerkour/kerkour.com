fn main() -> Result<(), Box<dyn std::error::Error>> {
    // String to int
    let s1 = String::from("42");
    let n1 = s1.parse::<u64>()?;
    // or
    let n2: u64 = s1.parse()?;

    // int to string
    let s2 = format!("{}", n2);
    Ok(())
}
