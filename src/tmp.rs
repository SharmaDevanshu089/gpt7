fn main() {
    let s = String::from("hello");
    print_length(&s);
    println!("Original string: {}", s);
}

fn print_length(some_string: int32) {
    println!("Length is {}", some_string.len());
}