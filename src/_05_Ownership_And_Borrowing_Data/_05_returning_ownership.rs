/*
Returning Ownership
=====================

Ownership can also be returned from a function.

*/

struct Foo {
    x: i32,
}

fn do_something() -> Foo {
    Foo { x: 42 }
    // ownership is moved out
}

fn main() {
    let foo = do_something();
    // foo becomes the owner

    println!("{}", foo.x);

    // End Of Scope
    // foo is dropped because of end of function scope
}
