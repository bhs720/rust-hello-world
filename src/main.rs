use std::fmt::{Debug, Display};

fn main() {
    // print_i32(112);
    // print_str("hello world!");
    // print_thing::<i32>(42);
    // print_thing::<&str>("cheeseburgers");
    // print_thing(21);
    // print_thing("oof");
    print_things(42, "the answer");
}

fn print_things<A, B>(thing_a: A, thing_b: B) where A: Display, B: Display {
    println!("thing_a: {}", thing_a);
    println!("thing_b: {}", thing_b);
}

fn print_thing<T: std::fmt::Display>(thing: T)
{
    println!("The thing is a {}: {}", std::any::type_name::<T>(), thing);
}

fn print_i32(num: i32) {
    println!("The number is {num}");
}

fn print_str(the_string: &str) {
    println!("The string is {the_string}");
}