/*
Basic Type Conversion
=====================

Rust requires explicitness when it comes to numeric types.
One cannot use a u8 for a u32 casually without error.

Luckily Rust makes numeric type conversions very easy with
the 'as' keyword.
*/

fn main() {
    let a = 13u8;
    let b = 7u32;
    let c = a as u32 + b;
    println!("{}", c);

    let t = true;
    println!("{}", t as u8);
    println!("{}", t as i8);

    print!("-------------------------------\n");
}
