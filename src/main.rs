use std::io::{self, Read};

// https://doc.rust-lang.org/std/io/fn.stdin.html
fn main() -> io::Result<()>  {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;
    Ok(println!());
}
