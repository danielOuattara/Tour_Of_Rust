/* 
Ownership
==========

Instantiating a type and binding it to a variable name 
creates a memory resource that the Rust compiler will 
validate through its whole lifetime. 

The bound variable is called the resource's owner. */


struct Foo {
    x: i32,
}

fn main() {
    // We instantiate structs and bind to variable foo
    // to create memory resources
    let foo = Foo { x: 42 };
    
    // foo is the owner

    println!("{}",foo.x );
}
