/*
Ownership is a system  used to manage memory. Set rules are used by the compiler for checks 
at compiletime. 

Stack and Heap - Refresher
Stack and Heap are both areas of memory your code can use at runtime. Stack stores value in a LIFO
Queue. This is for data that has a known fixed size at compile time. Data that has an unknown size
at compile time, or that may change in size is stored on the heap. OS finds space in the heap and
returns a pointer. Pointer is then stored on the stack, as it is a fixed size. Heap access is
slower. 

When code calls a function, values passed into the function (potentially including pointers to 
data on the heap) and the functions local variables get pushed onto the stack. When the function
exits, the values are popped off the stack.

Ownership takes care of tracking what parts of code are using data on the heap, minimizing the 
amount of duplicate data on the heap, and cleaning up unused data on the heap so you dont run
out of space.

Ownership Rules:
	- Each value in Rust has a variable that is called its OWNER
	- There can only be one owner at a time
	- When the owner goes out of scope, the value will be dropped

Variable Scope
scope is the range within a program for which an item is valid. for example
*/

    let s = "hello";

/*
the variable s refers to a string literal that is hardcoded it is valid from the point it is 
declared until the end of its scope.
*/

{                      // s is not valid here, it’s not yet declared
    let s = "hello";   // s is valid from this point forward

    // do stuff with s
}                      // this scope is now over, and s is no longer valid

/*
String Type
Rust has a type 'String' that is allocated on the heap and is able to store variable amounts
of text. Can create a String from a string literal using the 'from' function
*/

     let s = String::from("hello");

/*
the :: operator is an operator that allows us to namespace this particular from function under
the String type rather than using some sort of name like 'string_from'. This type of string can
be mutated. 

What is the difference here?
With a string literal, the size is known at compiletime, so the value is hardcoded into
the binary. It is fast and efficient, but also immutable. When we need a String that we can
change, we need to use a 'String' instead. 

In order to support a growable, mutable piece of text, we need to allocate memory on the heap, 
of an unknown amount at compile time to hold the contents. This means:
    -The memory must be requested from the operating system at runtime
    -We need a way to return this memory to the OS when were done with our String

the first part is included in the implementation of String::from. this is universal in programming languages.

The second part varies from language to language. With Rust, the memory is automatically returned
once the variable that owns it goes out of scope. Rust calls a special function called drop
automatically at the closing curly bracket.

Lets discuss this in the context of some common ways multiple variables can interact.

MOVE:
*/

let x = 5;
let y = x;

/*
assigns the value of x to y. This is simple because integers are simple values with known fixed sizes. 

With a string its not that simple. A string variable is actuall bound to a structure containing a 
pointer to the data in the heap, the length of the string, and the capacity of the allocated
memory.

when we assign a string to another variable many other languages copy this structure instead.
This is known as a shallow copy. both point to the same area in memory. However this could lead
to a double free which is dangerous. Instead Rust considers s1 no longer valid. 
*/

let s1 = String::from("hello");
let s2 = s1;

println!("{}, world!", s1);

/*
In this case, s1 would cause an error due to an invalidated reference. Functionally, s1 was
moved to s2. Rust will only free s2 when scope exits.

This also shows that rust will never create deep copies of your data. so automatic copying
can be assumed to be inexpensive in terms of runtime performance. 

CLONE:

If we do want to deeply copy the heap data of a string, not just the stack data, we can use
clone. for example:
*/

let s1 = String::from("hello");
let s2 = s1.clone();

println!("s1 = {}, s2 = {}", s1, s2);

/*
COPY:

Rust has an annotation called the 'Copy' trait that can be placed on types that are stored on the stack entirely. If a type has the copy trait, an older variable is still usable after assignment
like the example provided using ints above. Rust wont let us annotate a type with the copy trait 
if the type or any of its parts has implemented the drop trait.

Ownership and Functions

Passing a variable into a function will move or copy just as assignment does.
*/

fn main() {
    let s = String::from("hello");  // s comes into scope

    takes_ownership(s);             // s's value moves into the function...
                                    // ... and so is no longer valid here

    let x = 5;                      // x comes into scope

    makes_copy(x);                  // x would move into the function,
                                    // but i32 is Copy, so it’s okay to still
                                    // use x afterward

} // Here, x goes out of scope, then s. But because s's value was moved, nothing
  // special happens.

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.

//Return values can also transfer ownership.

fn main() {
    let s1 = gives_ownership();         // gives_ownership moves its return
                                        // value into s1

    let s2 = String::from("hello");     // s2 comes into scope

    let s3 = takes_and_gives_back(s2);  // s2 is moved into
                                        // takes_and_gives_back, which also
                                        // moves its return value into s3
} // Here, s3 goes out of scope and is dropped. s2 goes out of scope but was
  // moved, so nothing happens. s1 goes out of scope and is dropped.

fn gives_ownership() -> String {             // gives_ownership will move its
                                             // return value into the function
                                             // that calls it

    let some_string = String::from("hello"); // some_string comes into scope

    some_string                              // some_string is returned and
                                             // moves out to the calling
                                             // function
}

// takes_and_gives_back will take a String and return one
fn takes_and_gives_back(a_string: String) -> String { // a_string comes into
                                                      // scope

    a_string  // a_string is returned and moves out to the calling function
}

