use std::fs::File;


fn main() {
    // panic!("crash and burn");

    let v = vec![1,2,3];
    // v[99];
    let greeting_file_result = File::open("files/hello1.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => println!("Hello World"),
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };

}

enum Result1<T,E> {
    Ok(T),
    Err(E),
}