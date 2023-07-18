pub fn variables() {
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

pub fn changing_variables() {
    let mut value = 4;
    println!("{value:? }");

    value = 42;
    println!("{value:? }");

    print!("-------------------------------\n");
}

pub fn basic_types() {
    let x = 12; // by default this is i32
    let a = 12u8;
    let b = 4.3; // by default this is f64
    let c = 4.3f32;
    let bv = true;
    let t = (13, false);
    let sentence = "hello world!";
    println!(
        "{} {} {} {} {} {} {} {}",
        x, a, b, c, bv, t.0, t.1, sentence
    );

    print!("-------------------------------\n");
}

pub fn basic_conversion() {
    let a = 13u8;
    let b = 7u32;
    let c = a as u32 + b;
    println!("{}", c);

    let t = true;
    println!("{}", t as u8);
    println!("{}", t as i8);

    print!("-------------------------------\n");
}

const PI: f32 = 3.14159;

pub fn constants() {
    println!("To make an apple {PI} from scratch, you must first create a universe.");

    print!("-------------------------------\n");
}

pub fn arrays() {
    let array_numbers: [i32; 3] = [1, 2, 3];
    println!("{array_numbers:?}");
    println!("{}", array_numbers[1]);

    print!("-------------------------------\n");
}

//---------------------------------------------

fn add(x: i32, y: i32) -> i32 {
    return x + y;
}

fn subtract(x: i32, y: i32) -> i32 {
    x + y
}

pub fn functions_1() {
    println!("42 + 13 = {}", add(42, 13));
    println!("42 - 13 = {}", subtract(42, 13));
    print!("-------------------------------\n");
}

//---------------------------------------------

fn swap(x: i32, y: i32) -> (i32, i32) {
    return (y, x);
}

pub fn functions_2() {
    // multiple return values: return a tuple of return values

    let result = swap(123, 456);
    println!("result.0 = {}, result.1 = {}", result.0, result.1);

    // destructure the previous tuple into 2 variables names
    let (a, b) = swap(result.0, result.1);
    println!("a = {} b = {}", a, b);

    print!("-------------------------------\n");
}

//---------------------------------------------
// If no return type is specified for a function, it returns an empty tuple, also known as a unit.
//An empty tuple is represented by ().

fn make_nothing_1() {
    return ();
}

// the return type is implied as ()
fn make_nothing_2() {
    // this function will return () if nothing is specified to return
}

pub fn functions_3() {
    // return nothing

    let a = make_nothing_1();
    let b = make_nothing_2();

    // Printing a debug string for a and b; because it's hard to print nothingness
    println!("The value of a: {:?}", a);
    println!("The value of b: {:?}", b);

    print!("-------------------------------\n");
}
