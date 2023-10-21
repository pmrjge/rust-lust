use std::cmp::PartialOrd;
use std::fmt::Display;

use generic_types_traits_lifetimes::news_article::{Summary, Tweet};

fn main() {
    // Removing Duplication By Extracting a Function
    let number_list = vec![35, 50, 25, 100, 65];
    let mut largest = &number_list[0];

    for number in &number_list {
        if number > largest {
            largest = number;
        }
    }

    println!("The largets number is {largest}");

    let number_list = vec![102, 34, 6000, 89, 53, 2, 43, 8];

    let mut largest = &number_list[0];

    for number in &number_list {
        if number > largest {
            largest = number;
        }
    }

    println!("The largest number is {largest}");

    let number_list = vec![102, 32, 6000, 89, 54, 2, 43, 8];

    println!("The largest number is {}", largest(&number_list));

    // Generic Data Types
    // In Function Definitions
    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest_char(&char_list);
    println!("The largest char is {result}");

    let integer = Point {x: 5, y: 10};
    let float = Point {x: 1.0, y:4.0};

    let both_integer = Point0 {x:5, y:10};
    let both_float = Point0 {x: 1.0, y: 4.0};
    let integer_and_float = Point0{x:5, y: 4.0};

    // In Enum Definitions
    // In method definitions
    println!("p.x = {}", integer.x());

    let p1 = Point0 {x: 5, y: 10.4};
    let p2 = Point0 {x: "Hello", y: 'c'};
    let p3 = p1.mixup(p2);
    println!("p3.x = {}, p3.y={}", p3.x, p3.y);

    // Performance of Code Using Generics
    //Traits
    // Implementing a trait on a type
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize())

    // Default implementations
    // Traits as Parameters
    // Trait bound syntax
    // Specifying multiple trait bounds with the + Syntax
    // Clearer trait bounds with where clauses

    // Using Trait Bounds to Conditionally Implement methods

    // Validating References With Lifetimes
    // Lifetime annotation syntax
    fn long_est<'a>(x: &'a str, y: &'a str) -> &'a str {
        if x.len() > y.len() {x} else {y}
    }
    // thinking in terms of lifetimes
    fn longest<'a>(x: &'a str, y: &str) -> &'a str {
        x
    }

    // Lifetime annotations in struct definitions
    struct ImportantExcerpt<'a> {
        part: &'a str,
    }
    let novel = String::from(
        "Call me Ishmael. Some years ago..."
    );
    let first_sentence = novel
        .split('.')
        .next()
        .expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };

    // Lifetime Elision
    // Lifetime annotations in method definitions
    // The static lifetime
    let s: &'static str = "I have a static lifetime.";
    // Generic Types Parameters, Trait Bounds and LifeTimes Together
    fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T,)-> &'a str
    where T: Display
    {
        println!("Announcement! {ann}");
        if x.len() > y.len() {
            x
        } else { y }
    }
}


/*fn returns_summarizable(switch: bool) -> impl Summary {
    if switch {
        // something
    }
    else {
        // something else
    }
}*/


struct Point0<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point0<T, U> {
    fn mixup<V, W>(self, other: Point0<V, W>) -> Point0<T, W> {
        Point0 {
            x: self.x,
            y: other.y
        }
    }
}

struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T>{
    fn x(&self) -> &T {
        &self.x
    }
}

impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

fn find_largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if largest > item {
            largest = item;
        }
    }
    largest
}

fn largest(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_char(list: &[char]) -> &char {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}


