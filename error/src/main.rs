use std::fs::File;
use std::io::ErrorKind;
use std::io::{self, Read};

fn main() {
    let greeting_file_result = File::open("helo.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {e:?}"),
            },
            other_error => {
                panic!("Problem opening the file: {other_error:?}");
            }
        },
    };

    println!("{:#?}", read_username_from_file().unwrap());
}

fn read_username_from_file() -> Result<String, io::Error> {
    let mut username = String::new();

    File::open("helo.txt")?.read_to_string(&mut username)?;

    Ok(username)
}
