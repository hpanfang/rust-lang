/*
The match control flow operator

match is a control flow operator that allows you to compare
a value against a series of patterns and then execute code 
based on which pattern matches

Here is an example of match:
*/

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

/*
Note the code associated with each arm is an expression and 
the resulting value is what gets returned for the entire
match expression

If code is long, you can use curly brackets, as shown below
*/

fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

/*
Another useful feature of match arms is that they can bind 
to the parts of the values that match the pattern. This is 
useful for extracting values out of enum variants.
*/

fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        },
    }
}

/*
Option <T> is an enum in rust that is part of the standard 
library. It is very useful, as it is the only way to 
represent null in rust. it has two possible values, some<T> 
and none<T>. where <T> is a generic that can be any type. 

You can use Option<T> in a match statement as well, as shown
below:
*/

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

let five = Some(5);
let six = plus_one(five);
let none = plus_one(None);

/*
Matches are also exhaustive. The rust compiler knows every possible
case for a given match expressiona and will throw an error if there
is a case that is not covered.

the _ can be used as a placeholder.
*/

let some_u8_value = 0u8;
match some_u8_value {
    1 => println!("one"),
    3 => println!("three"),
    5 => println!("five"),
    7 => println!("seven"),
    _ => (),
}

//_ matches any value. can be used as the default at the bottom of a match

