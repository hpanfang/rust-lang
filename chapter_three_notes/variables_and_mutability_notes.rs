//Example of immutable variable error. Cannot reassign an immutable variable
fn main() {
    let x  = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
}

//Correct way to achieve this:
fn main() {
    let mut x  = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
}

//const vs immutable variable
//const is always immutable, declared with const, and the value must be annotated.
//can be declared at any scope, and the value is set to constant expression.
const MAX_POINTS: u32 = 100_000;

//Shadowing is declaring new variable with same name as previous variable
//requires 'let', allows you to perform transformations on immutable variables
fn main() {
    let x = 5;

    let x = x + 1;

    let x = x * 2;

    println!("The value of x is: {}", x);
}
//also allows you to mutate type
let spaces = "   ";
let spaces = spaces.len();

//if you were to try using mut instead you would get an error.
//because you cant mutate a variables type
let mut spaces = "   ";
spaces = spaces.len();