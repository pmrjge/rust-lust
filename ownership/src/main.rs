fn main() {
    let _s = "hello";
    let mut s = String::from("hello");
    s.push_str(", world!");
    println!("{}", s);

    let x = 5;
    let _y = x;

    let s1 = String::from("hello");
    let _s2 = s1;

    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {s1}, s2 = {s2}");

    let x = 5;
    let y = x;
    println!("x = {x}, y={y}");

    // Ownership and Functions
    let s = String::from("hello");

    takes_ownership(s);

    let x = 5;
    makes_copy(x);
    let s1 = gives_ownership();
    let s2 = String::from("hello");
    let s3 = takes_and_gives_back(s2);

    let s1 = String::from("hello");
    let (s2, len) = calculate_length(s1);
    println!("The length of '{s2} is {len}.");

    // References and Borrowing
    let s1 = String::from("hello");
    let len = compute_length(&s1);
    println!("The length of '{s1} is {len}");

    // Mutable references
    let mut s = String::from("hello");
    change(&mut s);

    {
        let r1 = &mut s;
    }
    let r2 = &mut s;

    let mut s = String::from("hello");
    let r1 = &s;
    let r2 = &s;
    println!("{r1} and {r2}");

    let r3 = &mut s;
    println!("{r3}");

    // Data References
}

fn change(some_string: &mut String) {
    some_string.push_str(",world");
}

fn compute_length(s: &String) -> usize {
    s.len()
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();

    (s,length)
}

fn gives_ownership() -> String {
    let some_string = String::from("yours");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}


fn takes_ownership(some_string: String) {
    println!("{some_string}");
}

fn makes_copy(some_integer: i32){
    println!("{some_integer}");
}