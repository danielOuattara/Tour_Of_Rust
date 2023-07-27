/*
Failable Main
==============

main method has the capability of returning a Result!

*/

fn do_something_that_might_fail(i: i32) -> Result<f32, String> {
    if i == 42 {
        Ok(13.0)
    } else {
        Err(String::from("this is not the right number"))
    }
}

// Main returns no value, but could return an error!

fn main() -> Result<(), String> {
    let result_1 = do_something_that_might_fail(12);

    match result_1 {
        Ok(v) => println!("found {v}"),
        // Err(e) => println!("Error: {}", e), // Working !
        Err(_e) => {
            // println!("Error: {}",_e)
            //OR
            // handle this error gracefully
            // OR
            // return a new error from main that said what happened!
            return Err(String::from("something went wrong in main!"));
            
        }
    }

    // Notice we use a unit value inside a Result Ok
    // to represent everything is fine
    Ok(())
}
