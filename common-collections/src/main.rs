fn main() {
    // Creating a Vector
    let v: Vec<i32> = Vec::new();
    let v = vec![1, 2, 3];

    // Updating a vector
    let mut v = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    // reading elements from vectors
    let v = vec![1, 2, 3, 4, 5];
    let third: &i32 = &v[2];
    println!("The third element is {third}");

    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    };

    let v = vec![1,2,3,4,5];
    let does_not_exist = v.get(100);

    // Iterating over the values in a vector

    let v = vec![100, 32, 57];
    for i in &v {
        println!("{i}");
    }

    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }

    // Using an Enum to store multiple types

    enum SpredsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![SpredsheetCell::Int(3), SpredsheetCell::Text(String::from("blue")), SpredsheetCell::Float(10.12)];

    // Dropping a Vector Drops Its Elements

    {
        let v = vec![1,2,3,4]

        // manipulate v

    } // from here v is released from memory

    // Storing UTF-8 encoded text with strings
    // Creating a new string
    let mut s = String::new();

    let data = "initial contents";
    let s = data.to_string();
    let s = "initial contents".to_string();

    let s = String::from("initial contents");
    let hello = String::from("السلام عليكم");
    let hello = String::from("Dobrý den");
    let hello = String::from("Hello");
    let hello = String::from("שָׁלוֹם");
    let hello = String::from("नमस्ते");
    let hello = String::from("こんにちは");
    let hello = String::from("안녕하세요");
    let hello = String::from("你好");
    let hello = String::from("Olá");
    let hello = String::from("Здравствуйте");
    let hello = String::from("Hola");

    // Updating a string with push_str and push
    let mut s = String::from("foo");
    s.push_str("bar");

    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {s2}");

    let mut s = String::from("lo");
    s.push('l');

    // Concatenation with the + operator or the format! Macro
    let s1 = String::from("Hello, ");
    let s2 = String::from("world");
    let s3 = s1 + &s2;

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{s1}-{s2}-{s3}");

    // Indexing into strings
    // Iterating over strings
    for c in "Зд".chars() {
        println!("{c}");
    }

    for b in "Зд".bytes() {
        println!("{b}");
    }

    // Hash Maps
}
