/*
Representing Nothing
=====================

In other languages, the keyword 'null' is used to represent
an absence of a value. It creates difficulty in programming
languages because it creates the possibility that our program
might fail when interacting with a variable/field.

Rust does not have 'null', but it is not ignorant of the
importance of representing nothing!

Consider a naive representation using a tool we already know.

This pattern of providing a 'None' alternative representation
for one or many alternate values is so common in Rust because
of its lack of a 'null' value. Generic types help solve this
challenge.
*/

#![allow(dead_code)]

enum Item {
    Inventory(String),
    None, // None represents the absence of an item
}

struct BagOfHolding {
    item: Item,
}

fn main(){

    let _none_item = BagOfHolding {
        item: Item::None,
    };

    let _not_none_item = BagOfHolding{
        item: Item::Inventory(String::from("cellphone")),
    };


}