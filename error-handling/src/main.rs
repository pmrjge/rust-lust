use std::fs::{self,File};
use std::io::{self, Read, ErrorKind};

fn main() {
    // panic!("crash and burn");

    let v = vec![1,2,3];
    // v[99];
    let greeting_file_result = File::open("files/hello1.txt");

    /*let greeting_file = match greeting_file_result {
        Ok(file) => println!("Hello World"),
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };*/

    // Matching on Different Errors
    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => {
                match File::create("files/hello.txt") {
                    Ok(fc) => fc,
                    Err(e) => panic!("Problem creating the file {:?}", e),
                }
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error);
            }
        }
    };
    //Alternatives to using match with Result<T, E>

    let greeting_file1 = File::open("files/hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    } );

    // Shortcut for panic on error: unwrap and expect
    let greeting_file = File::open("hello.txt").expect("hello.txt should be included in this project");

    // Propagating Errors
    // A shortcut for propagating errors: The ? Operator
    // Where the ? operator can be used

    // To Panic or not to panic!

}

enum Result1<T,E> {
    Ok(T),
    Err(E),
}

fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }

}

fn read_username_from_file1() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}

fn read_username_from_file2() -> Result<String, io::Error> {
    let mut username = String::new();

    File::open("hello.txt")?.read_to_string(&mut username)?;
    Ok(username)
}

fn read_username_from_file_short() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}

fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}