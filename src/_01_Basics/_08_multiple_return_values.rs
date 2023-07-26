/*
Multiple Return Values
=======================

Functions can return multiple values by returning
a tuple of values.

Tuple elements can be referenced by their index
number.

Rust supports various kinds of destructuring that
we will see in many forms, allowing us to extract
sub-pieces of data structures in ergonomic ways.
*/

//---------------------------------------------

fn swap(x: i32, y: i32) -> (i32, i32) {
    return (y, x);
}

//---------------------------------------------

pub fn main() {
    // multiple return values: return a tuple of return values

    let result = swap(123, 456);
    println!("result.0 = {}, result.1 = {}", result.0, result.1);

    // destructure the previous tuple into 2 variables names
    let (a, b) = swap(result.0, result.1);
    println!("a = {} b = {}", a, b);

    print!("-------------------------------\n");
}
