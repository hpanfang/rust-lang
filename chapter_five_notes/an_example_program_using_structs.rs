/*
What follows is an example program demonstrating the
usefulness of structs, starting with single variables and 
then refactoring to use structs instead.
*/

fn main() {
    let width1 = 30;
    let height1 = 50;

    println!(
        "The area of the rectangle is {} square pixels.",
        area(width1, height1)
    );
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

/*
This works but doesnt make sense. We are calculating the 
area of one rectangle, but our area function takes two 
related but seperate parameters. This relationship should
be expressed to make our code more readable and managable. 

We could do this with Tuples:
*/

fn main() {
    let rect1 = (30, 50);

    println!(
        "The area of the rectangle is {} square pixels.",
        area(rect1)
    );
}

fn area(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

/*
This gives us more structure, and we are now only passing one argument. but tuples dont name their elements and so we have to index into the tuple. this makes our code less readable. 
We need to use structs to add lables to the data. 
*/

struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle { width: 30, height: 50 };

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

//How to print out structs:

//Cant use println! with substitution a la 
    println!("rect1 is {}", rect1);

/*
the curly brackets in this statement tell println! to use a formatting
known as Display. However, structs do not provide an implementation for
display, so this will not work

putting a :? specifier in the curly brackets tells println! we want to use
an output format called Debug. However, this will still fail, as we need to opt 
into this in order to make it available. we need to add an annotation prior to our 
struct as shown below.
*/

#[derive(Debug)]

//you can also use :#? to format the output differrently.

//RUst has provided a number of traits to use with the derive annotation