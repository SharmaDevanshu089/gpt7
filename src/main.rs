fn main() {
    let _number = 6;
    print!("The Final number is {}", double_if_even(_number));
    print!("{}", _number);
}
fn double_if_even(number : i32) -> i32{
    let new_number;
    if number %2 == 0{
        new_number= number*2;
    }
    else {
        new_number = number;
    }
    return new_number;
}