use handson2::min_max::solve;
use std::error::Error;
use std::io;
use std::io::Read;

fn main() -> Result<(), Box<dyn Error>>{
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let output = solve(&input)?;
    print!("{}", output);
    Ok(())
}
