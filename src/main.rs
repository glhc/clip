use std::io::{self, Read, Write};
use std::fs::{self, DirBuilder};

// https://doc.rust-lang.org/std/io/fn.stdin.html
fn main() {
    let input = read_stdin()
        .unwrap()
        .to_string();

    
    // if there is content coming from stdin, write it to:
    // /var/lib/clip/clipstorage
    if check_input_for_content(&input) == true {
       /car 
    };
    let write_result = write_stdout(input);
    assert_eq!(write_result.is_err(), false);
    
    //TODO make a fn to create the clipstore file
    //TODO complete fn to create the clip directory
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

fn check_input_for_content(&input) -> bool {
    if &input == "" {
        false
    } else {
        true
    }
}

fn create_clip_dir() {
    let path = "/var/log/lib/clip"

    // Create the folder
    DirBuilder::new()
        .recursive(true)
        .create(path)
        .unwrap();

    assert!(fs::metadata(path).unwrap().id_dir());
}
