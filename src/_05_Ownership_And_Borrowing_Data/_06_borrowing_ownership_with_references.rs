/*
Borrowing Ownership with References
====================================

References allow us borrow access to a resource
with the '&' operator.

References are also dropped like other resources.

*/

struct Foo {
    x: i32,
}

fn main() {
    let foo = Foo { x: 42 };

    let f = &foo;
    println!("{}", f.x);

    println!("{}", foo.x);

    // End Of Scope:
    // f is dropped here
    // foo is dropped here
}
