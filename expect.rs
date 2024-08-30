use std::collections::HashMap;

#[derive(Debug)]
enum MyError{
    Error1
}

fn divide(dividend: i32, dividor: i32) -> Result<i32, MyError>{
    if dividend % dividor != 0{
        Err(MyError::Error1)
    } else {
        Ok(dividend / dividor)
    }
}

fn main() {
    let divide = divide(3, 2);
    let res = divide.expect("We Crashed");

    println!("{}", res);
}