/*main function entry point of programs*/
/*fn keyword used to declare new functions*/

/*Rust code uses snake case for function and variable names. this means letters are lowercase and
underscores seperate words*/

/*program with example function definition*/

fn main() {
    println!("Hello, world!");

    another_function();
}

fn another_function() {
    println!("Another function.");
}

/*note definition location does not matter*/

/*functions can have parameters, values provided as parameters are arguments*/

/*example of function with parameters*/

fn main() {
    another_function(5, 6);
}

fn another_function(x: i32, y: i32) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}

/* Rust is an expression-based language. This means there is a distinction between statements and
expressions.

Statements - instructions that perform some action but do not return a value
Expressions - instructions that evaluate to a a resulting value.

Ex. creating a variable and assigning it a value with the let keyword is a statement. let y = 6;
function definitions are also statements. Statements dont return values so you can assign a let
statement to another variable. for example, the following would error out:

fn main() {
    let x = (let y = 6);
}

expressions evaluate to something and make up most of the rust code. ex. 5 + 6 is an expression that
returns the value 11.

expressions can be part of statements. for example, the 6 in let y = 6; is an expression that
evaluates to the value of 6.

Calling a function is an expression. calling a macro is an expression. the block that we use to
create new scopes, {}, is an expression. for example:

    let y = {
        let x = 3;
        x + 1
    };

the expression

{
    let x = 3;
    x + 1
}

is a block that evaluates to 4. note that x + 1 has no semicolon at the end. if it did, that would
turn it into a statement, thus not returning a value. */


/*Functions with return values. dont name return values, but declare type after an ->
in rust return value is the value of the final expression in to block of the body of the function
can return early from function with return keyword and a specified value, though most functions just
return implicitly. for example:

fn five() -> i32 {
    5
}

fn main() {
    let x = five();

    println!("The value of x is: {}", x);
}