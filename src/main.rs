use std::io::{self, Read, Write};

// https://doc.rust-lang.org/std/io/fn.stdin.html
fn main() {
    let input = read_stdin()
        .unwrap()
        .to_string();
    
    let write_result = write_stdout(input);
    assert_eq!(write_result.is_err(), false);
    
}

fn read_stdin() -> io::Result<String> {
    let mut buffer = String::new();
    let stdin = io::stdin();
    let mut handle = stdin.lock();

    handle.read_to_string(&mut buffer)?;

    Ok(buffer)
}

fn write_stdout(input: String) -> io::Result<()> {
    let stdout = io::stdout();
    let mut handle = stdout.lock();
    let output = input.into_bytes();
    
    handle.write_all(&output);

    Ok(())
}
