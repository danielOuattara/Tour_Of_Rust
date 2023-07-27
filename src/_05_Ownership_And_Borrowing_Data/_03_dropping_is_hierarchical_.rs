/* 
Dropping is Hierarchical
=========================

When a struct is dropped, the struct itself is dropped 
first, then its children are dropped individually, and 
so on.

Memory Details:

- By automatically freeing memory Rust helps ensure that 
  there are fewer memory leaks.

- Memory resources can only be dropped once.

 */


struct Bar {
    x: i32,
}

struct Foo {
    bar: Bar,
}

fn main() {
    let foo = Foo { bar: Bar { x: 42 } };
    println!("{}", foo.bar.x);

    // End Of Scope
    // foo is dropped first
    // then foo.bar is dropped
}
