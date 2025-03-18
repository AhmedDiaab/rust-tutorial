fn main() {
    println!("main function");
    another_function();
    another_function_w_ps(5, 'B'); // 'B' not "B"

    // statement vs expression
    let x = 5; // statement

    // expression "in a simple words" is anything that can return a value

    let y = {
        let z = 3;
        z + 1 // see here no simi colon, that means this line will pass value to y
    };

    let z = another_function_w_ps_rt(y);

    println!("value of y is {}", y);
    println!("value of z is {}", z);

}

// fn definition is a statement
fn another_function() {
    println!("another function");
}

fn another_function_w_ps(x: u8, letter: char) {
    println!("another function: {x}{letter}");
}

fn another_function_w_ps_rt(x: u8) -> u8 {
    x + 1 // if we put ; it will return from expression to statement
}
// dont return a value = dont evaluate a value