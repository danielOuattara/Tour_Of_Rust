/*
while
=======

while lets you easily add a condition to a loop.

If the condition evaluates to false, the loop will exit. */

fn main() {
    let mut x = 0;
    while x != 7 {
        println!("x is {}", x);
        x += 1;
    }
    println!("-----------------------");
}
