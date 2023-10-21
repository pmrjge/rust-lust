use std::fs::File;
use std::io::ErrorKind;

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


}

enum Result1<T,E> {
    Ok(T),
    Err(E),
}