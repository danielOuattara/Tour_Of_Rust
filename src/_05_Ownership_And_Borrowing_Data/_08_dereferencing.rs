/* 
Dereferencing
==============

Using '&mut' references, you can get the owner's value 
using the * operator.

You can also get a copy of an owned value using the * 
operator, if the value can be copied (we will discuss 
copyable types in later chapters). 

*/

fn main() {
    let mut foo = 42;
    let f = &mut foo;
    let bar = *f; // get a copy of the owner's value
    *f = 13;      // set the reference's owner's value
    println!("{}", bar);
    println!("{}", foo);
}
