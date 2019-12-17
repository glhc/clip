use std::error::Error;
use std::io::{self, Read, Write};
use std::fs::{self, DirBuilder, File};

// https://doc.rust-lang.org/std/io/fn.stdin.html
fn main() {
    let input = read_stdin()
        .unwrap()
        .to_string();


    if input.is_empty() {
        //read clipstore and
        let clipstore = read_clipstore().unwrap();
        write_stdout(&clipstore);

        //write clipstore to stdout.

    };
    
}

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

// fn create_clipstore() -> Result<(())> {
//     let path = "/var/lib/clip";

//     // Create the folder
//     DirBuilder::new()
//         .recursive(true)
//         .create(path)
//         .unwrap();

//     assert!(fs::metadata(path).unwrap().is_dir());

//     let mut file = File::create("/var/lib/clip/clipstore")?;
    

//     Ok(());
//     Err("Oops");
// }

fn read_clipstore() -> Result<String, io::Error>{
    let mut f = File::open("/var/lib/clip/clipstore")?;
    let mut s = String::new();
    f.read_to_string(&mut s);
    Ok(s)
}

// Wipes the clipstore and writes the new stuff
fn write_clipstore(input: &String) {
    let clipstore_path = "/var/lib/clip/clipstore";
    let mut clipstore = fs::write(clipstore_path, input);
}
