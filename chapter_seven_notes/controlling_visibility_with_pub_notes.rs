/*
In rust, all code is default by private. If you dont use a private function in your program
Rust will throw a warning, as your program is the only one allowed to use said function.

You can declare functions as public to avoid this. this lets rust know that the function will be used
by code outside your program.

To mark a function as public we use the pub keyword at the start of the declaration.
*/

pub mod client

/*
privacy rules
-if an item is public, it can be accessed through any of its parent modules
-if an item is private, it can be accessed only by its immediate parent module
and any of the parents child modules.

example, which of the following will throw errors?
*/

mod outermost {
    pub fn middle_function() {}

    fn middle_secret_function() {}

    mod inside {
        pub fn inner_function() {}

        fn secret_function() {}
    }
}

fn try_me() {
    outermost::middle_function();           //no error
    outermost::middle_secret_function();    //error
    outermost::inside::inner_function();    //error
    outermost::inside::secret_function();   //error
}

/*
The try_me function is in the root module of our project. The module named outermost 
is private, but the second privacy rule states that the try_me function is allowed to
access the outermost module because outermost is in the current (root) module, as is try_me.
*/