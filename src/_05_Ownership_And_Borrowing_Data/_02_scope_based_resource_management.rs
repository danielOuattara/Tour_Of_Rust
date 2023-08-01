/*
Scope-Based Resource Management
================================

Rust uses the end of scope as the place to deconstruct
and deallocate a resource.

The term for this deconstruction and deallocation is
called a 'drop'.

Memory detail:

- Rust does not have garbage collection.
- This is also called Resource Acquisition Is Initialization ( RAII ) in C++.
 */

struct Foo {
    x: i32,
}

fn main() {
    let foo_a = Foo { x: 42 };
    let foo_b = Foo { x: 13 };

    println!("{}", foo_a.x);

    println!("{}", foo_b.x);

    // End Of Scope:
    // foo_b is dropped here
    // foo_a is dropped here
}
