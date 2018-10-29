//Struct definition:

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

/*
To use a struct after it has been defined we create an instance of the struct 
by specifying concrete values for each of the fields as shown below

fields dont need to be specified in the same order in which they were
declared. 
*/

let user1 = User {
    email: String::from("someone@example.com"),
    username: String::from("someusername123"),
    active: true,
    sign_in_count: 1,
};

/*
specific values can be retrieved using dot notation
if the instance is mutable, the value can be changed using dot notation as
well. Note that the whole instance must be mutable, you cant just mark certain
fields in the struct as mutable.

example of a function that takes an email and a username and returns a user instance
*/

fn build_user(email: String, username: String) -> User {
    User {
        email: email,
        username: username,
        active: true,
        sign_in_count: 1,
    }
}

/*
It makes sense to name the function parameters the same as the struct fields
but having to repeat the email and username field names and variables is a bit
tedious. 

Field Init Shorthand - when variables and fields have the same name
we can omit the repetition as shown below.
*/

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

/*
Creating a new instance from other instances with Struct Update syntax. You can specifying
the values for different fields, and then use the .. syntax xto show that the remaining 
fields should have the same value as the given instance.
*/

let user2 = User {
    email: String::from("another@example.com"),
    username: String::from("anotherusername567"),
    ..user1
};

/*
Tuple structs

Yout can define structs that look similar to tuples, called tuple structs. Tuple structs have
the added meaning the struct name provides but dont have names associated with their fields
they just have typtes of the fields. These are useful when you want to give the whole tuple a name
and make the tuple a different type from other tuples. 

these are defined by the struct keyword and struct name followed by the types in the tuple for example
*/

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

let black = Color(0, 0, 0);
let origin = Point(0, 0, 0);

/*
These have the same value but are not the same as they are considered distinct types. Other than this they
behave like normal tuples.

you can also define structs that dont have any fields. they are called unit-like structs
because they behave similarly to (), the unit type. These are more relevant later on when we
discuss traits.

For now, we will not store references to data, as we want the instances of the struct to own all of its data and
for that data to be valdi as long as the whole struct is valid. We can pass references in a struct instance. This requires lifetimes, a 
rust feature we will discuss later.
*/