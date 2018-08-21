
use std::io;

fn main() {
    println!("Enter index in fibonacci sequence (indexing starts at 0)");
    let mut index = String::new();

    io::stdin().read_line(&mut index)
        .expect("Failed to read line");

    let index: i32 = match index.trim().parse() {
        Ok(num) => num,
        Err(_) => return,
    };

    let result = recursive_fibonacci(index);
    println!("the fibonacci digit at index {} is {}", index, result);
}

fn recursive_fibonacci(index: i32) -> i32 {
    if index <= 1 {
        index
    } else {
        recursive_fibonacci(index - 1) + recursive_fibonacci(index - 2)
    }
}