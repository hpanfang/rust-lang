/*
You can also create a library crate project instead of a binary crate project. a library crate can be pulled into projects as a dependency. The example below is a skeleton of a library that provides some general networking functionality called 'communicator'. 

to create a library, pass the --lib option as follows: 'cargo new communicator --lib'
This will generate a src/lib.rs instead of src/main.rs. Since there is no main file, there is nothing to be executed. So instead of using cargo run, we will use cargo build.

Module Definitions
For our example, we will define a module named network that contains the definition of a function called connect. every module in rust starts with the mod keyword.
*/

mod network {
	fn connect() {
	}
}

/* 
everything within the curly brackets following network is within the network namespace
for example, to call the connect() function outside the network module, we would need
to specify the module and use namespace syntax:
*/

network::connect()

//you can have multiple mods side by side in the same lib.rs file. for example:

mod network {
	fn connect() {
	}
}

mod client {
	fn coonnect() {
	}
}

//since the two functions are inside different namespaces there is no conflict.
//These modules could also be nested if that was more logical

mod network {
    fn connect() {
    }

    mod client {
        fn connect() {
        }
    }
}

//These functions still have seperate namespaces (network:connect, network:client:connect())

/*
Modules form a hierarchical structure, and we can seperate them into multiple files.
for example, the following code: 
*/

mod client {
    fn connect() {
    }
}

mod network {
    fn connect() {
    }

    mod server {
        fn connect() {
        }
    }
}

/*
This code has the following hierarchy
communicator
	L client
	L network
	 	L server

We can break these out into their own files so that it makes more sense from an 
organization perspective.

for example, we can replace the client code with only the declaration of the client module
*/

mod client;

mod network {
    fn connect() {
    }

    mod server {
        fn connect() {
        }
    }
}

/*
We are still declaring the client module here, but the semicolon tells rust to look in
another location for the code defined within the scope of the client module.
This would require creating an external file called client.rs and define our client module
code in there. we dont need a mod declaration because the client mod has already been 
declared in src/lib.rs. Rust only looks in src/lib.rs by default. if you want to add more
modules to the project, you need to tell Rust to look in other files. Thats why mod client; needs
to be defined in src/lib.rs

This will not work for submodules. For example for the server module, we would need to 
-make a new directory called network
-move our extracted network module, network.rs, into this directory as mod.rs
-move our extracted server module, server.rs, into this directory.

This allows rust to distinguish that server is supposed to be a submodule of network.

Rules of Module Filesystems
-if a module named foo has no submodules, you should put declarations for foo in a file named
foo.rs
-if a module named foo does have submodules, you should put declarations for foo in a file named
foo/mod.rs

these rules are recursive
*/