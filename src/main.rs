use std::fmt::Display;

fn main() {
    // print_i32(112);
    // print_str("hello world!");
    // print_thing::<i32>(42);
    // print_thing::<&str>("cheeseburgers");
    // print_thing(21);
    // print_thing("oof");
    // print_things(42, "the answer");
    // let my_num = get_number();
    // println!("Got number: {my_num}");
    // println!("first is {}", first((11, 22)));
    // let mut my_var = 0;
    // println!("my_var is: {}", my_var);
    // mutate_var(&mut my_var);
    // println!("my_var is: {}", my_var);
    fizz_buzz();
}

fn fizz_buzz() {
    for n in 1..50 {        
        let m3 = n % 3 == 0;
        let m5 = n % 5 == 0;
        
        if m3 {
            print!("Fizz");
        }

        if m5 {
            print!("Buzz");
        }

        if (m5 || m3) == false {
            print!("{n}");
        }

        println!("");
    }
}

// fn mutate_var(x: &mut i32) {
//     *x = 42;
// }

// // parameter is a tuple
// fn first((value, _): (i32, i32)) -> i32 { value }

// fn get_number() -> i32 {
//     #![doc = "Return the number 42"]
//     return 42;
// }

// fn print_things<A, B>(thing_a: A, thing_b: B) where A: Display, B: Display {
//     println!("thing_a: {}", thing_a);
//     println!("thing_b: {}", thing_b);
// }

// fn print_thing<T: std::fmt::Display>(thing: T)
// {
//     println!("The thing is a {}: {}", std::any::type_name::<T>(), thing);
// }

// fn print_i32(num: i32) {
//     println!("The number is {num}");
// }

// fn print_str(the_string: &str) {
//     println!("The string is {the_string}");
// }
