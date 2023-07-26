/*
Constants
==========

Constants allow us to specify a common value that's
used throughout our code many times efficiently.
instead of copying values like variables where they
are used, constants directly replace the text
identifier where they are used with their value at
compile time.

Unlike variables, constants must always have explicit
types.

Constant names are always in SCREAMING_SNAKE_CASE. */

const PI: f32 = 3.14159;

fn main() {
    println!("To make an apple {PI} from scratch, you must first create a universe.");
    print!("-------------------------------\n");
}
