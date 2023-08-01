/*

Borrowing Mutable Ownership with References
============================================

We can also borrow mutable access to a resource with
the '&mut' operator.

A resource owner cannot be moved or modified while
mutably borrowed.

Memory detail:

- Rust prevents having two ways to mutate an owned value
  because it introduces the possibility of a data race.

*/

struct Foo {
    x: i32,
}

fn do_something(f: Foo) {
    println!("{}", f.x);
    // f is dropped here
}

fn main() {
    let mut foo = Foo { x: 42 };
    let bar = &mut foo;

    // do_something(foo); // ERR : foo cannot be moved while mutably borrowed

    // foo.x = 13; // ERR: foo is not modifiable while mutably borrowed

    bar.x = 13;
    // bar is dropped here because it's no longer used after this point

    bar.x = 17; // ?? why it is working ??

    println!("{}", foo.x);

    // this works now because all mutable references were dropped
    foo.x = 7;

    // move foo's ownership to a function
    do_something(foo);
}
