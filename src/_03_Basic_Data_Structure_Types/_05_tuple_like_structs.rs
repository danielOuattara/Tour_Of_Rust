/*
Tuple Like Structs
===================

For conciseness, you can create structs that are used like a tuple.

*/

struct Coordinates(i32, i32);

fn main() {
    let coordinates_1 = Coordinates(5, 47);

    println!("{}, {}", coordinates_1.0, coordinates_1.1);
}
