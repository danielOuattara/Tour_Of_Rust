/*
if/else
========

Code branching in Rust is not surprising.

Conditions don't have parentheses! Did we ever
really need them? Our logic now looks nice and clean.

All your usual relational and logical operators still
 work: ==, !=, <, >, <=, >=, !, ||, &&.

*/

fn main() {
    let x = 42;
    if x < 42 {
        println!("less than 42");
    } else if x == 42 {
        println!("is 42");
    } else {
        println!("greater than 42");
    }
}
