fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    let home = IpAddrV2::V4(String::from("127.0.0.1"));
    let loopback = IpAddrV2::V6(String::from("::1"));

    let home = IpAddrV3::V4(127, 0, 0, 1);
    let loopback = IpAddrV3::V6(String::from(":.1"));

    let m = Message::Write(String::from("hello"));
    m.call();

    // The option Enum and Its Advantages over Null Values
    let some_number = Some(5);
    let some_char = Some('e');
    let absent_number: Option<i32> = None;

    // The match Control Flow Construct
    value_in_lucky_cents(Coin::Penny);

    // Patterns that Bind To Values
    value_in_cents_states(Coin1::Quarter(UsState::Alaska));

    // Matching with Option<T>
    let five = Some(5);
    let six = plus_one(five);
    let seven = plus_one(six);

}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        Option::None => Option::None,
        Some(i) => Some(i + 1),
    }
}

fn value_in_cents_states(coin:Coin1) -> u8 {
    match coin {
        Coin1::Penny => 1,
        Coin1::Nickel => 5,
        Coin1::Dime => 10,
        Coin1::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        },
    }
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

enum Coin1 {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_lucky_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky Penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

enum IpAddrKind {
    V4,
    V6,
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

enum IpAddrV2 {
    V4(String),
    V6(String),
}

enum IpAddrV3 {
    V4(u8, u8, u8, u8),
    V6(String),
}

enum Message {
    Quit,
    Move {x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {

    }
}