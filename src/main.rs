fn main() {
    print_i32(112);
    print_str("hello world!");
    print_thing::<i32>(42);
    print_thing::<&str>("cheeseburgers");
    print_thing(21);
    print_thing("oof");
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