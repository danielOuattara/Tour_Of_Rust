/*
 Creating Data in Memory
=======================

When we 'instantiate' a struct in our code our program creates
the associated field data side by side in memory.

We instantiate by specifying all field values within
StructName { ... }.

Struct fields are then accessed using a dot operator '.'

Memory details of our example:
------------------------------

* The text inside the quotes is read only data (e.g. "Ferris"),
therefore it is placed in the 'data memory' region.

* The function call 'String::from()' creates a struct field of
type 'String' that is placed side by side with the fields of
SeaCreature in the 'stack memory'.
A String represents text that can be changed and does this by:
    1- Creating memory on the heap for the text where it can be
        modified
    2- Storing a reference to that memory location on the heap
        and storing it in String struct (more on this in future
        lessons)

* Finally our two friends 'Ferris' and 'Sarah' have data structures
that will always have fixed locations in our program, so they are
placed on the stack memory.
*/

struct SeaCreature {
    animal_type: String,
    name: String,
    arms: i32,
    legs: i32,
    weapon: String,
}

pub fn main() {
    let ferris = SeaCreature {
        animal_type: String::from("crab"),
        name: String::from("ferris"),
        arms: 2,
        legs: 4,
        weapon: String::from("claw"),
    };

    println!(
        "{} is a {}. They have {} arms, {} legs, and a {} weapon",
        ferris.name, ferris.animal_type, ferris.arms, ferris.legs, ferris.weapon
    );

    let sarah = SeaCreature {
        animal_type: String::from("octopus"),
        name: String::from("Sarah"),
        arms: 8,
        legs: 0,
        weapon: String::from("brain"),
    };

    println!(
        "{} is a {}. They have {} arms, and {} legs. They have no weapon..",
        sarah.name, sarah.animal_type, sarah.arms, sarah.legs
    );
}
