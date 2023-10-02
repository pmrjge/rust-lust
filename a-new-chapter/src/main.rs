fn main() {
    // Functions
    println!("Hello, world!");

    another_function();

    // Parameters
    another_fn(5);
    print_labeled_measurement(5, 'h');
    // Statements and Expressions
    let y = expression();
    println!("{y}");

    // Functions with Return Values
    fn five() -> i32{
        5
    }

    let x = five();
    println!("The value of x is {x}");

    fn plus_one(x: i32) -> i32 {
        x + 1
    }

    let w = plus_one(x);
    println!("{w}");

    // Control Flow
    let number = 3;
    if number < 5 {
        println!("Condition was true");
    } else {
        println!("Condition was false");
    }

    if number > 0 {
        println!("Condition was true");
    }

    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by four");
    } else if number % 3 == 0 {
        println!("number is divisible by three");
    } else if number % 2 == 0 {
        println!("number is divisible by two");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    let condition = true;
    let number = if condition {5} else {6};
    println!("The value of number is: {number}");

    // Repetition with Loops
    loop {
        println!("again");
        break;
    }

    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");

    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }

            remaining -= 1;
        }
        count += 1;
    }
    println!("End count = {count}");

    let mut number = 3;
    while number != 0 {
        println!("{number}!");
        number -= 1;
    }

    println!("LIFTOFF!!!");

    let a = [10, 20, 30, 40, 50];
    let mut index = 0;
    while index < 5 {
        println!("The value is {}", a[index]);
        index += 1;
    }

    for element in a {
        println!("the value is {element}");
    }

    for number in (1..4).rev() {
        println!("{number}");
    }

    println!("LIFTOFF!!!");
}

fn another_function(){
    println!("Another function.");
}

fn another_fn(x: i32){
    println!("The value of x is: {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char){
    println!("The measurement is: {value}{unit_label}");
}

fn expression() ->i32 {
    let y = {
        let x = 3;
        x + 1
    };
    y
}