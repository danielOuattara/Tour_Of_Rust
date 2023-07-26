/* loop
========

Need an infinite loop?
Rust makes it easy.
break will escape a loop when you are ready.
loop has a secret we'll talk about soon. */

fn main() {
    let mut x = 0;
    loop {
        if x == 9 {
            break;
        }
        println!("{}", x);
        x += 1;
    }
    println!("-----------------------");
}
