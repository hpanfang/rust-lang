/* 
Methods are similar to functions. They are declared with the 
fn keyword and their name, they can have parameters and a 
return value. They contain code that is run when theyre 
called from somewhere else. 

Methods are different in that they are defined within the
context of a struct (or an enum or a trait object). The 
first parameter is always self, which represents the 
instance of the struct the method is being called on.

To define a function in the context of the Rectangle struct, 
we start with an impl (implementation) block. the area 
function will be defined within the block, with the first 
and in this case, the only parameter being self. we then 
call this method with dot notation. 
*/

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let rect1 = Rectangle { width: 30, height: 50 };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
} 

/*
We use &self in the signature for area instead of &Rectangle 
because the type of self can be inferred due to being in the 
impl block for Rectangle. Methods can take ownership of 
self, borrow self immutably as weve done here, or borrow
self mutably, just like any other param. 

Impl blocks also allow us to define functions that dont take
self as a paramater. These are called associated functions 
because they are associated with the struct. They are still 
functions, not methods, because they dont have an instance 
of the struct to work with. Associated functions are often 
used for constructors that will return a new instance of the
struct. For example.
*/

impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }
}

/*
To call this, we use the :: syntak with the struct name:
let sq = Rectangle::square(3);

You can have multiple impl blocks for a single struct. This 
is not always useful but it is valid syntax.
*/