/*
Moving Ownership
=================

When an owner is passed as an argument to a function,
ownership is moved to the function parameter.

After a move the variable in the original function can
no longer be used.

Memory details:

- During a move the stack memory of the owners value is
  copied to the function call's parameter stack memory.

*/

struct Foo {
    x: i32,
}

fn do_something(f: Foo) {
    println!("{}", f.x);

    // End Of Scope
    // f is dropped here
}

fn main() {
    // foo is created.
    let foo = Foo { x: 42 };
    println!("{}", foo.x);

    // foo is moved to do_something() and handle there
    do_something(foo);

    // foo can no longer be used
    // println!("{}", foo.x) // ERROR
}
