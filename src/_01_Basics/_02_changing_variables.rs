/*
Changing Variables
==================

Rust cares a great deal about what variables are modifiable.
Values fall into two types:

- mutable - the compiler will allow the variable to be written to and read from.
- immutable - the compiler will only allow the variable to be read from.

Mutable values are denoted with a 'mut' keyword.

We will have more to say on this concept later, but for now
just keep an eye out for this keyword.
 */

fn main() {
    let mut value = 4;
    println!("{value:? }");

    value = 42;
    println!("{value:? }");

    print!("-------------------------------\n");
}
