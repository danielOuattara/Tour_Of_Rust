/*

References Of References
=========================

References can even be used on pieces of references. */

struct Foo {
    x: i32,
}

fn do_something(a: &Foo) -> &i32 {
    return &a.x;
}

fn main() {
    let mut foo = Foo { x: 42 };
    println!("{}", foo.x);

    let x = &mut foo.x;
    ù * x = 13;
    // x is dropped here allow us to create a non-mutable reference

    let y = do_something(&foo);
    println!("{}", y);
    // y is dropped here
    // foo is dropped here
}
