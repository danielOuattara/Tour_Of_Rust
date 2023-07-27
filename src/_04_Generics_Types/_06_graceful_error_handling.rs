/* 
Graceful Error Handling
========================

Result is so common that Rust has a powerful operator '?' 
for working with them. These two statements are equivalent:

1)
do_something_that_might_fail()?

2)
match do_something_that_might_fail() {
    Ok(v) => v,
    Err(e) => return Err(e),
}
 */

fn do_something_that_might_fail(i: i32) ->Result<f32, String>{
    if i == 42 {
        Ok(13.0)
    } else {
        Err(String::from("This is not the right number"))
    }
}

fn main() -> Result<(), String> {
    // We save much code
    let v = do_something_that_might_fail(42)?;
    println!("found {}", v);

    let w = do_something_that_might_fail(12)?;
    println!("found {}", w);
    
    Ok(())
}

