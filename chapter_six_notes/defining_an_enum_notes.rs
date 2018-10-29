/*Defining an Enum

As an example for the usefulness of enums, consider ip 
addresses (which have two major versions). It would be 
useful for us to enumerate all possible values for ip 
addresses in out program.

Enum values can only be one of multiple variants. You can 
express this function in code by defining an enumeration as 
shown below. 
*/

enum IpAddrKind {
    V4,
    V6,
}

/*
V4 and V6 are known as enum variants. IpAddrKind is now
a custom data type

you can create instances of the two variants as follows:
*/

let four = IpAddrKind::V4;
let six = IpAddrKind::V6;

//You could create a function that takes this data type:

fn route(ip_type: IpAddrKind) { }

//and call said function with either variant:

route(IpAddrKind::V4);
route(IpAddrKind::V6);

//You acn also attach data to each variant of the enum directly
//like so:

enum IpAddr {
    V4(String),
    V6(String),
}

let home = IpAddr::V4(String::from("127.0.0.1"));

let loopback = IpAddr::V6(String::from("::1"));

/*
Taking this even further, you can actually specify different
types and amounts of associated data per variant as shown 
below:
*/

enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

let home = IpAddr::V4(127, 0, 0, 1);

let loopback = IpAddr::V6(String::from("::1"));

/*
You can associate any kind of data you want with a variant,
even a struct.

Additionally, you can define methods on enums within an impl 
block just like with structs.
*/