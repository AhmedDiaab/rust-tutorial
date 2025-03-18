fn main() {
    // data types: scalar types, compound types

    // scalar types: integers, floating point numbers, characters, boolean

    // integers
    let mut x: u8 = 57;
    let y = 57u8; // type suffix form of x

    let hex_example = 0xffff; // we can type any number in hex, binary, or octal

    let character_example = b'a'; // b stands for byte literal and it is ascii representation for letters
    println!("{}, {},{hex_example}", x, y);
    println!("{character_example}");

    // integer overflow happen when variable exceeds size that is defined
    x += 198;
    println!("{x}");
    // x += 1;
    // println!("{x}"); // error thrown here because max number for u8 is 255 and it has value of 256

    // to handle integer overflow, refer to this: https://doc.rust-lang.org/book/ch03-02-data-types.html#integer-overflow

    // floating points

    let f1: f32 = 2.0;
    let f2: f64 = 3.0; // this rely on modern cpus, same speed of f32 but more percesion

    println!("{f1}, {f2}");

    // numeric operations
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Results in -1

    // remainder
    let remainder = 43 % 5;

    println!("{sum},{difference},{product},{quotient},{truncated},{remainder}");

    // TODO: study operators

    // boolean
    let bool_example = true;

    println!("{bool_example}");

    // characters
    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';
    println!("{c},{z}, {heart_eyed_cat}");

    // compound types: tuples, arrays
    // tuples

    let tuple_example: (u8, f32, bool) = (1, 3.0, true); // cannot be printed directly
    let (t1, t2, t3) = tuple_example; // used pattern matching to destructure tuple values
    println!("{},{},{}", t1, t2, t3);

    // another way to access tuple elements
    let t11 = tuple_example.0;
    println!("{}", t11);

    let unit = (); // special type of tuple, called unit

    // arrays
    let a = [1, 2, 3, 4]; // fixed type, fixed space

    let a = [3; 2]; // we can create array out of a single element replicated among the array elements, = [3,3]

    println!("{}", a[0]); // when access index not exist, an error out of bounds will be thrown

}
