//control flow with if let

/*
lets you combine if and let into a less verbose way to
handle values that match one pattern while ignoring the rest.

For example, the following match statement
*/

let some_u8_value = Some(0u8);
match some_u8_value {
    Some(3) => println!("three"),
    _ => (),
}

//can be streamlined to the following

if let Some(3) = some_u8_value {
    println!("three");
}

/*
However, you will lose the exhaustive check that match 
enforces.

you can also include an else with an if let, which behaves 
like the default case in a match. 
*/

let mut count = 0;
if let Coin::Quarter(state) = coin {
    println!("State quarter from {:?}!", state);
} else {
    count += 1;
}

//if else is basically syntax sugar for a specific match.