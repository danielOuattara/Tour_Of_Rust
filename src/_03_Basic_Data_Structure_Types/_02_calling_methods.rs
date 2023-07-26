/*
 Calling Method
===============

Unlike 'functions', 'methods' are functions associated with
a specific data type.

- 'static methods' belong to a type itself and are called
   using the '::' scope resolution operator.

- 'instance methods' belong to an instance of a type are
   called using the '.' operator.
*/

pub fn main() {
    // Using a static method to create an instance of String
    let my_string = String::from("Hello Rust !");

    // Using a method on the instance
    println!("{} is {} characters long.", my_string, my_string.len());
}
