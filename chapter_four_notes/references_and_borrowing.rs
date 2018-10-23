Taking and returning ownership with every function can be a bit tedious.
What if we want to let a function use a value but not take ownership? We use a reference.

& used to indicate a reference, which allows you to refer to some value without taking ownerhip
of it. The reference is actually a pointer to the pointer object that is pointing to the data on the heap. The opposite of referencing is dereferencing which is indicated by *.

example within the context of a function: 


let s1 = String::from("hello");

let len = calculate_length(&s1);

&s1 syntax lets us creata a reference that refers to the value of s1 but does not own it. 
the value it points to will not be dropped when the reference goes out of scope.

the function signature uses & to indicate that the type of the parameter s is a reference


fn calculate_length(s: &String) -> usize { // s is a reference to a String
    s.len()
} // Here, s goes out of scope. But because it does not have ownership of what
  // it refers to, nothing happens.

references are immutable by default. can be made mutable. to be mutable, the original variable
must be mutable, and then the reference can be typed as mutable as well. However we can only
have one mutable reference to a particular piece of data in a given scope. This control allows
rust to prevent data races at compile time.

additionally, you cannot combine immutable and mutable references to a particular piece of data
for a given scope. this is because immutable references do not expect values to change. 

Code that can create dangling refrences will also get caught at cocmpile time. 
