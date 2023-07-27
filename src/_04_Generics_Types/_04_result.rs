/*
Result
=======

Rust has a built in generic enum called 'Result' that
allows us to return a value that has the possibility
of failing. It is the idiomatic way in which the language
does error handling.

enum Result<T, E> {
    Ok(T),
    Err(E),
}

Note that our generics type has multiple parameterized
types separated by a comma.

This enum is so common that, instances of the enum can
be created anywhere with the enum variants Ok and Err.

*/

fn do_something_that_might_fail(i: i32) -> Result<f32, String> {
    if i == 42 {
        Ok(13.0)
    } else {
        Err(String::from("this is not the right number"))
    }
}

fn say_my_name(name: String) -> Result<String, String> {
    if name == "john" {
        Ok(String::from("You found it !"))
    } else {
        Err(String::from("this is not the right name"))
    }
}

fn main() {
    // match lets us deconstruct Result elegantly and ensure we handle all cases!

    let result = do_something_that_might_fail(12);
    match result {
        Ok(v) => println!("found {}", v),
        Err(e) => println!("Error: {}", e),
    }

    let result_2 = do_something_that_might_fail(42);
    match result_2 {
        Ok(v) => println!("found {}", v),
        Err(e) => println!("Error: {}", e),
    }

    //------

    let match_name = say_my_name("anna".to_owned());
    match match_name {
        Ok(v) => println!("found {}", v),
        Err(e) => println!("Error: {}", e),
    }

    let match_name_2 = say_my_name("john".to_owned());
    match match_name_2 {
        Ok(v) => println!("found {}", v),
        Err(e) => println!("Error: {}", e),
    }
}
