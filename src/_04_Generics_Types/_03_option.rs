/*
Options
=========

Rust has a built in generic enum called 'Option' that 
allows us to represent nullable values without using 
null.

enum Option<T> {
    None,
    Some(T),
}

This enum is so common, instances of the enum can
be created anywhere with the enum variants Some and
None.
*/


// A partially defined struct type
// Our parameter type T can be handed to others
struct BagOfHolding<T> {
    item: Option<T>,
}

fn main() {
    // Note: A bag of type i32, holding nothing! We have to specify the type
    // because otherwise Rust would not know what type of bag it is.

    let i32_empty: BagOfHolding<i32> = BagOfHolding::<i32> { item: None };

    if i32_empty.item.is_none() {
        println!("This item contents nothing ");
    } else {
        println!("This item contents something")
    }

    //--------

    let i32_42: BagOfHolding<i32> = BagOfHolding::<i32> { item: Some(42) };

    if i32_42.item.is_none() {
        println!("This item contents nothing ");
    } else {
        println!("This item contents something: {:?}", i32_42.item )
    }

    //--------

    // match lets us deconstruct Option elegantly and ensure we handle all cases!
    match i32_empty.item {
        Some(v) => println!("found {} in bag!", v),
        None => println!("found nothing"),
    }
    
    match i32_42.item {
        Some(v) => println!("found {} in bag!", v),
        None => println!("found nothing"),
    }
}
