/*
What is a String?

The String type, which is provided by Rusts standard library, is a growable, mutable
owned UTF-8 encoded string type. 

Rusts standard library also includes a number of other string types, such as OsString, OsStr,
CString, Cstr. These can store text in different encodings or be represented in memory in a different
way. 

Creating a New String
*/

let mut s = String::new(); //creating a new empty string

//if we have some initial data, we use the to_string method

let data = "initial contents";

let s = data.to_string();

//the method also works on a literal directly
let s = "initial contents".to_string();

//can also use String::from to creat a string from a literal
let s = String::from("initial contents");

//A String can grow and change. For example, the + operator or format! macro

let s1 = String::from("Hello, ");
let s2 = String::from("world!");
let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used

let s1 = String::from("tic");
let s2 = String::from("tac");
let s3 = String::from("toe");

let s = format!("{}-{}-{}", s1, s2, s3);

//A String can be grown with the push_str method to append a string slice as shown below

let mut s = String::from("foo");
s.push_str("bar");

//the push method takes a single character as a parameter and adds it to the String

let mut s = String::from("lo");
s.push('l');

/*
Rust strings do not support indexing. The reason for this deals with how Rust stores strings
in memory.

A string is actually a wrapper over a Vec<u8>. This is to account for 2 byte characters in unicode
this means that an index value may not always correspond to the full unicode scalar value. Theres a lot
of other reasons too.

If you really want a string slice you can use index syntax with a range to grab a slice.
This can still mess things up though. For example.
*/


let hello = "Здравствуйте";

let s = &hello[0..1];

//will return

"thread 'main' panicked at 'byte index 1 is not a char boundary; it is inside 'З' (bytes 0..2) of `Здравствуйте`', src/libcore/str/mod.rs:2188:4"

//the chars method seperates a string out and returns each character as a char value. these results can then be iterated over.

