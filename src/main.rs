use std::io::{self, Read};

// https://doc.rust-lang.org/std/io/fn.stdin.html
fn main() {
    let input = read_stdin()
        .unwrap()
        .to_string();

    println!("{}", input);
}

fn read_stdin() -> io::Result<String> {
    let mut buffer = String::new();
    let stdin = io::stdin();
    let mut handle = stdin.lock();

    handle.read_to_string(&mut buffer)?;
    Ok(buffer)
}

fn write_stdout() {

}
