/*
the type HashMap<K, V> stores a mapping of keys of type K to values of type V.
This is useful when you want to look up data not by using an index but with a key that could be
of any type. 

Creating a new Hash Map - create wth new and add elements with insert. example below:
*/

use std::collections::HashMap;

let mut scores = HashMap::new();

scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Yellow"), 50);

/*
HashMap is less ubiquitous. so its not automatically brought into scope in the prelude.
It does not have built-in constructor macros either. 

Hash maps are homogenous, all keys must be the same type and all values must be the same type
as well.

You can also construct a hash map by using the collect method on a vector of tuples, where each
tuple consists of a key and its value. the example below uses the zip method to create a vector of tuples
and then the collect method turns this into a hash map.
*/

use std::collections::HashMap;

let teams  = vec![String::from("Blue"), String::from("Yellow")];
let initial_scores = vec![10, 50];

let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();

/*
Hash maps and ownership

for types that implement the copy trait, the values are copied into the hash map
for owned values like String, the values will be moved and the hash map will be the owner of 
those values.

We can get values out o fthe hash map by providing its key to the get method.
we can iterate over each key value pair in a hash map in a similar manner as a vector, with a for loop
*/

use std::collections::HashMap;

let mut scores = HashMap::new();

scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Yellow"), 50);

for (key, value) in &scores {
    println!("{}: {}", key, value);
}

/*
Updating a hash map:

If we insert a key and value into a hash map and then insert that same key with a different
value, the original value will be replaced.

hash map has a special API called entry that takes a key you want to check as a parameter. 
The return value of the entry method is an enum called Entry that represents a value that might or might not
exist. the or_insert method will return a mutable reference to the corresponding entry key if that key exists
or inserts the parameter as athe new value for this key and returns a mutable reference to the new value
for example 
*/

use std::collections::HashMap;

let mut scores = HashMap::new();
scores.insert(String::from("Blue"), 10);

scores.entry(String::from("Yellow")).or_insert(50);
scores.entry(String::from("Blue")).or_insert(50);

println!("{:?}", scores);

//This mutable reference also allows us to modify the returned value. 

use std::collections::HashMap;

let text = "hello world wonderful world";

let mut map = HashMap::new();

for word in text.split_whitespace() {
    let count = map.entry(word).or_insert(0);
    *count += 1;
}

println!("{:?}", map);
