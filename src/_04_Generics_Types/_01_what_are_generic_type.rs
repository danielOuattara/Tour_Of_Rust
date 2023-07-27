/*
What Are Generic Types ?
=========================

Generic types allow us to partially define a struct or enum,
enabling a compiler to create a fully defined version at
compile-time based off our code usage.

Rust generally can infer the final type by looking at our
instantiation, but if it needs help you can always be explicit
using the ::<T> operator, also known by the name turbofish operator.
*/

#![allow(dead_code)] // this line prevents compiler warnings
#![allow(unused)]


// A partially defined struct type
struct BagOfHolding<T> {
    item: T,
}

fn main() {
    /* Note: by using generic types here, we create compile-time
    created types. Turbofish operator lets us be explicit. */

    let i8_item = BagOfHolding::<i8> { item: 42 };
    let i32_item = BagOfHolding::<i32> { item: 42 };
    let i64_item = BagOfHolding::<i64> { item: 42 };

    let boolean_item = BagOfHolding::<bool> { item: true };
    
    let string_item = BagOfHolding::<String> {
        item: "super book".to_owned(),
    };
    let another_string_item = BagOfHolding::<String> {
        item: String::from("super book"),
    };

    let f32_item = BagOfHolding::<f32> { item: 3.14 };
    let f64_item = BagOfHolding::<f64> { item: 3.14 };

    // -------------------  Rust can infer types for generics too!

    let i32_item_2 = BagOfHolding { item: 42 };
    let boolean_item_2 = BagOfHolding { item: true };
    let string_item_2 = BagOfHolding { item: "super book" };
    let float_item_2 = BagOfHolding { item: 3.14 };
    // Note: never put a bag of holding in a bag of holding in real life
    let bag_in_bag = BagOfHolding {
        item: BagOfHolding { item: "boom!" },
    };

    println!(
        "{} {} {} {}",
        i32_item.item, boolean_item.item, f32_item.item, bag_in_bag.item.item
    );
}