use std::error::Error;
use std::fs::{self, DirBuilder, File};
use std::io::{self, Read, Write};
use atty::Stream;

// https://doc.rust-lang.org/std/io/fn.stdin.html
fn main() {
    let input = read_stdin().unwrap();
    
    

    // If stin was piped here, save stdin
    if atty::isnt(Stream::Stdin) {
        // read clipstore
        let clipstore_contents = read_clipstore().unwrap();
        write_stdout(&clipstore_contents);
    // If stdin is a terminal (not piped), print clipstore
    } else {
        write_clipstore(&input);
    }
}

// TODO: don't wait for EOF when invoked and stdin is attached to a tty
fn read_stdin() -> io::Result<String> {
    let mut buffer = String::new();
    let stdin = io::stdin();
    let mut handle = stdin.lock();

    handle.read_to_string(&mut buffer)?;

    Ok(buffer)
}

fn write_stdout(input: &String) -> Result<(), Box<dyn Error>> {
    let stdout = io::stdout();
    let mut handle = stdout.lock();
    let output = input.as_bytes();
    handle.write_all(&output)?;

    Ok(())
}

fn read_clipstore() -> Result<String, io::Error> {
    let mut f = File::open("/var/lib/clip/clipstore")?;
    let mut s = String::new();
    f.read_to_string(&mut s);
    Ok(s)
}

// Wipes the clipstore and writes the new stuff
fn write_clipstore(data: &String) -> Result<(), io::Error> {
    println!("write_clipstore called");
    println!("input to store is: {}", data);
    let clipstore_dir = "/var/lib/clip/";
    let clipstore_path = "/var/lib/clip/clipstore";

    if !std::path::Path::new(clipstore_dir).is_dir() {
        create_clipstore();
    };

    // write expects a slice
    fs::write(clipstore_path, data)?; // perhaps .into_bytes?
                                      //TODO implement cargo test
                                      //TODO create docker rust environment
                                      //TODO create repl so i can test shit like fs::write makes a directory
    Ok(())
}

fn create_clipstore() -> Result<(), io::Error> {
    let clipstore_dir = "/var/lib/clip";
    DirBuilder::new()
        .recursive(true)
        .create(clipstore_dir)
        .unwrap();
    Ok(())
}
