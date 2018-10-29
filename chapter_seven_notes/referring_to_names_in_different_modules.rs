//weve covered calling functions defined in modules as shown below. however as you can 
//see this can get lengthy quickly.

pub mod a {
    pub mod series {
        pub mod of {
            pub fn nested_modules() {}
        }
    }
}

fn main() {
    a::series::of::nested_modules();
}

/*
Rust 'use' keyword shortens lengthy function calles by bringing the modules
of the function you want to call into scope.
*/

pub mod a {
    pub mod series {
        pub mod of {
            pub fn nested_modules() {}
        }
    }
}

use a::series::of;

fn main() {
    of::nested_modules();
}

/*
This keyword brings only what we have specified into scope. it does
not bring children of modules into scope. we still have to specify 
of:nested_modules when we want to call the nested modules function.
we could modify use to bring the function into scope instead.

You can also bring enum variants into scope as well. For any use statement
if you are bringing multiple items from one namespace into scope, you can list
them with curly brackets and commas in the last position like so:
*/

enum TrafficLight {
    Red,
    Yellow,
    Green,
}

use TrafficLight::{Red, Yellow};

fn main() {
    let red = Red;
    let yellow = Yellow;
    let green = TrafficLight::Green;
}

/*
You can bring all items in a namespace into scope at once with a * (glob)
These should be used with care. Example:
*/

enum TrafficLight {
    Red,
    Yellow,
    Green,
}

use TrafficLight::*;

fn main() {
    let red = Red;
    let yellow = Yellow;
    let green = Green;
}

/*
in rust, paths are always relative to the current module, except with a use
statement, which is relative to the root crate. to get a module in scope that is a sibling or parent 
module, we can use the following:
*/

::client:connect(); //leading colons say that we want to start from root
super::client::connect(); //super moves one up in module hierarchy from current

//these can be combined with use as well.
