/*Rust is a statically typed language - variable types are set at compile time.
Compiler can usually infer what type we want to use based on the value and how it is used.
when a variable could be multiple types (ex. converting a string to a numeric type) we need type annotation*/

let guess: u32 = "42".parse().expect("Not a number!");

//if we dont have this we get an error at compile time saying that we need to give guess a type

//Scalar Types
//represents a single value. four primary scalar types: Integer, Floating-point, Booleans, and characters

//Integer types - number w/o a fractional component

/*
Length	Signed	Unsigned
8-bit	i8	    u8
16-bit	i16	    u16
32-bit	i32	    u32
64-bit	i64	    u64
arch	isize	usize
*/
//signed can store -(2^(n-1)) to 2^(n-1)-1
//unsigned can store from 0 to 2^n - 1
//isize and usize is architecture based, so depends on hardware.

//Integer literals in rust: all (except byte literal) allow type suffix: 57u8, and '_' as a visual seperator such as 1_000
//more examples

/*
Number literals	Example
Decimal	        98_222
Hex	            0xff
Octal	        0o77
Binary	        0b1111_0000
Byte (u8 only)	b'A'
*/

//Floating point types - numbers with decimals. Rust supports two types: f32 and f64 which just indicate size
//f64 is default type because it is roughly as fast as f32 on modern CPUs but with more precision capability
//example of floating points:

fn main() {
    let x = 2.0; // f64

    let y: f32 = 3.0; // f32

//Rust supports basic mathematical operations as shown below

fn main() {
    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;

    // remainder
    let remainder = 43 % 5;
}

//Boolean Type - two possible values: true and false. specified via bool
//for example:

fn main() {
    let t = true;

    let f: bool = false; // with explicit type annotation
}

//Character type - primitive alphabetic type, specified with single quotes, strings use double quotes
//example:

fn main() {
    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';
}

//This type represents a unicode scalar value, which means a lot more than just ASCII

//Compound Types - group multiple values into one type. In rust these are tuples and arrays.
//below is an example of tuple creation with optional type annotations

fn main() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);
}

//tup is assigned to the tuple as a whole since it is a compound type.
//to get individual values out of a tuple, you can use pattern matching to destructure a tuple value
//ex:

fn main() {
    let tup = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("The value of y is: {}", y);
}

//The program creates a tuple, and then destructures it via let
//turning the tuple into three seperate variables
//can also access tuple element directly via period and index of desired element
//ex:

fn main() {
    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;

    let six_point_four = x.1;

    let one = x.2;
}

//Array type - elements of an array must have same type and arrays are fixed length. once declared cannot grow or shrink in size
//ex:

fn main() {
    let a = [1, 2, 3, 4, 5];
}

