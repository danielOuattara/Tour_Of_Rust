/*
Arrays
=======

An array is a fixed length collection of data elements
all of the same type.

The data type for an array is [T;N] where T is the
elements' type, and N is the fixed length known at
compile-time.

Individual elements can be retrieved with the [x]
operator where x is a usize index (starting at 0)
of the element you want.

Collections with a dynamic length, often called
dynamic or variable arrays, are introduced in a later
chapter about Vectors.
*/

fn main() {
    let array_numbers: [i32; 3] = [1, 2, 3];
    println!("{array_numbers:?}");
    println!("{}", array_numbers[1]);

    print!("-------------------------------\n");
}
