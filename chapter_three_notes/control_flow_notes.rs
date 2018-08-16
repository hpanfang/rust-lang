/* if expression allows you to branch code conditionally.

for example:
*/

fn main() {
    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }
}

//note, the condition must be a BOOL.Rust doesnt automatically convert non boolean types to a boolean
//rust also has optional if and else if statements

//since if is an expression, we can use it on the right side of a let statement. for example:
fn main() {
    let condition = true;
    let number = if condition {
        5
    } else {
        6
    };

    println!("The value of number is: {}", number);
}

//Rust has three kinds of loops: loop, while, and for

//loop tells rust to execute a block of code over and over until you explicitly tell it to stop
// via break or an external interrupt
fn main() {
    loop {
        println!("again!");
    }
}

//while is a conditional loop that runs while the condition is true.

fn main() {
    let mut number = 3;

    while number != 0 {
        println!("{}!", number);

        number = number - 1;
    }

    println!("LIFTOFF!!!");
}

//for allows you to execute some code for each item in a collection

fn main() {
    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("the value is: {}", element);
    }
}