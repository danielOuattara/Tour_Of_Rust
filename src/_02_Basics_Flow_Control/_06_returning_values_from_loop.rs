/*
Returning Values From loop
============================

loop can break to return a value.
 */

fn main() {
    let mut x = 0;

    let v = loop {
        x += 1;
        if x == 13 {
            break "found the 13";
        }
    };

    println!("from loop: {}", v);

    println!("-----------------------");
}
