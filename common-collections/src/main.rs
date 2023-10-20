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

}
