/*
The collection of type Vec<T> also known as a vector. Vectors allow you to store more than one
value in a single data structure that puts all the values next to each other in memory. 

Can only store values of the smae type.

Creating a New vector: to create a new empty vector, we call the
Vec::new function as shown below:
*/

let v: Vec<i32> = Vec::new();

/*
This gas a tyoe abbitation because we arent inserting any values
into this vector. Rust doesnt know what kind of elements we intend to store. If you provide
initial values, then this is not necessary. Rust provedes the vec! macro for convenience
This creates a new vector that holds the values you give it, as shown below:
*/

let v = vec![1, 2, 3];

/*
Updating a Vector

you can use the push method. as shown below. Since all the numbers pushed
are of type i32 rust can infer type from the data so we dont need a type annotation
*/

let mut v = Vec::new();

v.push(5);
v.push(6);
v.push(7);
v.push(8);

/*
Dropping a Vector

a vector is freed when it goes out of scope, and when a vector is free, its contents are also dropped

Reading Elements of Vectors

There are two ways to reference a value stored in a vector, with indexing syntax or the get method.
*/

let v = vec![1, 2, 3, 4, 5];

let third: &i32 = &v[2];
let third: Option<&i32> = v.get(2);

/*
the first method will cause a panic if there is a reference to a nonexistent element. This is best for 
when you want your program to crash if you are performing an out of bounds access.

The second metho returns a type of the Option enum. This means that your code will return None without panicing
in the instance where you access an out of bounds element.

Iterating over elements can be done with a for loop as shown below. 
*/

let v = vec![100, 32, 57];
for i in &v {
    println!("{}", i);
}

//or with a mutable reference, we can modify in a loop


let mut v = vec![100, 32, 57];
for i in &mut v {
    *i += 50;
}

//If you are looking to store a number of different types in a vector, create a enum. 
