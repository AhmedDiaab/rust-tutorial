fn main() {
    let number = 3;

    if number < 5 { // curly braces sometimes called arms
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    if number != 0 { // we cannot make condition like that, because condition must be evaluated to be boolean, fixed by converting "number" to "number != 0"
        println!("number was something other than zero");
    }

    let number = 6;

    if number % 4 == 0 { // use too many if..else make code clutter, so for cases like these then must use pattern matching
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    let condition = true;
    let number = if condition { 5 } else { 6 }; // can be used with let, and must with the same type (aka compatible), [this is an expression]

    println!("The value of number is: {number}");

    // repetitions: loop, for, while
    // loop: indefinite loop until i say stop (break)
    let mut iterations = 0;
    loop {
        if iterations > 10 {
            break;
        }
        println!("Again until reach 10, current: {}", iterations);
        iterations+=1;
    }

    // we can use loop as an expression
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2; // simicolon here optional
            // break vs return here: break ends loop while return value, and return returns from function
        }
    };

    println!("The result is {result}");

    inner_loops();

    conditional_loops();

    loop_collection();

    loop_collection_for();

    conditional_loops_v2();

}

fn inner_loops() {
    let mut count = 0;
    'counting_up: loop { // we can specify name for loops for multiple inner loops, aka setting lables to it
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
}

fn conditional_loops() {
    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");
}

fn loop_collection() {
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }
}

fn loop_collection_for() {
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }
}

fn conditional_loops_v2() {
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}