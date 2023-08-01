/*
Passing Around Borrowed Data
============================

Rust's rules for references might best be summarized by:

- Rust only allows there to be one mutable reference
  or multiple non-mutable references but not both.

- A reference must never live longer than its owner.

This doesn't tend to be a problem when passing around
references to functions.

Memory details:

- The first rule of references prevents data races.
  A data race when reading from data has the possibility
  of being out of sync due to the existence of a writer
  to the data at the same time. This happens often in
  multi-threaded programming.

- The second rule of references prevents the misuse of
  references that refer to non-existent data, called dangling
  pointers in C.

*/

struct Foo {
    x: i32,
}

fn do_something(f: &mut Foo) {
    f.x += 1;
    println!("{}", f.x);
    // mutable reference f is dropped here
}

fn main() {
    let mut foo = Foo { x: 42 };
    do_something(&mut foo);
    // because all mutable references are dropped within
    // the function do_something, we can create another.
    do_something(&mut foo);
    // foo is dropped here
}
