Slices

let you reference a contiguous sequence of elements rather than the whole collection

For example, how would you write a function that takes a string and returns the first
word it finds in that string (if it doesnt find a space in the string the whole string is
returned)

what would the function signature look like?

fn first_word(s: &String) -> //what do we return??

we could return the index of the end of the word

fn main() {
fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

This doesnt make much sense however as the value of our index is not tied to the word stored by S
if S gets cleared, our index is now useless.

String Slices are references to part of a string and look like this


fn main() {
let s = String::from("hello world");

let hello = &s[0..5];
let world = &s[6..11];
}

where [start..end] is a range that begins at start and continues up to but does not include end
if you do not include the start value, you begin at the first index, and if you do not include
the end index, you end at the length of the string.

if you do not include either value you get a slice equal to the entire string

the type that indicates a string slice is &str

rewriting the function from above: 

fn main() {
fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

Since the slice is a reference to the underlying data, we cant have errors like the ones mentioned
previously, as they will get caught by the compiler.

String literals are actually slices as well. They are a slice (&str) that points to the specific
point in the binary where the literal value is stored. This is why they are immutable, as &str
is an immutale reference.

Knowing this, we can also improve our function signature from before:

fn first_word(s: &str) -> &str {

now we can pass in string literals, or a slice referring to the entire string &s[..]

Slices in general work for all collections the same way string slices do. for example:


fn main() {
let a = [1, 2, 3, 4, 5];

let slice = &a[1..3];
}

This slice has type &i[32]
