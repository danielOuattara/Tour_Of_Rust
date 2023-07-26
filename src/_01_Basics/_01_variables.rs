/*
Variables are declared using the let keyword.

When assigning a value, Rust will be able to
infer the type of your variable 99% of the time.

If it cannot you may add the type to your variable
declaration.

Notice how we can assign to the same variable name
multiple times. This is called variable shadowing
and the type can be changed for subsequent references
to that name.

Variable names are always in snake_case. */

fn main() {
    // rust infers the type of x
    let user_age = 23;
    println!("{}", user_age);

    // rust can also be explicit about the type
    let pi_const: f64 = 3.14159;
    println!("{}", pi_const);

    // rust can also declare and initialize later, but this is rarely done
    let cash;
    cash = 0;
    println!("{}", cash);

    print!("-------------------------------\n");
}
