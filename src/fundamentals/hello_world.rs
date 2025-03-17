const X_Y_Z: &i32 = &123;

fn main() {
    // assigning variables
    let x = 5;
    let y = "test";

    println!("{x}, {y}");

    // mutability vs immutability
    let z = 3; // default is immutable for variables
    // z = 4; // error happened here because z is immutable
    // if we want more info about error we use this command `cargo --explain [error_code]`
    // example: `cargo --explain E0384`
    println!("{z}");

    // shadowing can be done to variables like this
    let z = 4; // reassign value
    println!("{z}");
    let z = "test2"; // we can change type too
    println!("{z}");

    // mutation
    let mut mutz = 2;

    println!("{mutz}");
    // we can reassign mutable variables
    mutz = 3;
    
    println!("{mutz}");
    
    // but we cannot change its type
    // mutz = "test";
    
    // println!("{mutz}");

    // constants
    println!("{X_Y_Z}");
    const X_Y_Z: &str = "123str"; // must specify type

    // can be existed and variable name is reserved in this block
    println!("{X_Y_Z}"); // as we can see when redeclare it is being shadowed from  globally scoped variable

}

// Question: why line 38 not printed 123 ?
// Question: Does shadowing applied to constants ?
