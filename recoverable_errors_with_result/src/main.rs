use std::fs::File;
use std::io::{self, ErrorKind, Read};


fn main() {
    let f = File::open("hello.txt");
    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Could not create file {:?}", e),
            },
            other_error => panic!("{:?}", other_error)
        }
    };

    let f2 = File::open("hello2.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello2.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });

    // this will cause panic if the file doesn't exist
    let f3 = File::open("hello3.txt").unwrap();

    // Same as above, but we can choose the error message
    let f4 = File::open("hello4.txt").expect("No good :(");
}

// propagating errors
fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello5.txt");
    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

// ? shortcut
fn read_username_from_file_with_shortcut() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?; // returns the Error if it fails
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}
