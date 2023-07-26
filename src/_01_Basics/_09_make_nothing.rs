/*
Returning Nothing
==================

If no return type is specified for a function,
it returns an empty tuple, also known as a unit.

An empty tuple is represented by ().
*/

fn make_nothing_1() {
    return ();
}

// the return type is implied as ()

fn make_nothing_2() {
    // this function will return () if nothing is specified to return
}

pub fn main() {
    // return nothing

    let a = make_nothing_1();
    let b = make_nothing_2();

    // Printing a debug string for a and b; because it's hard to print nothingness
    println!("The value of a: {:?}", a);
    println!("The value of b: {:?}", b);

    print!("-------------------------------\n");
}
