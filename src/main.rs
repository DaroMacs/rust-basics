const THISISA_CONSTANT: u32 = 100;
fn main() {
    immutability_test();
    scalar_types();
    shadowing();
    println!("The value of the constant is: {}", THISISA_CONSTANT);
}

fn immutability_test() {
    let x   = 1;   
    println!("The value of x is: {}", x);
    // x = 6; // This line would cause a compile-time error because x is immutable

    let mut y = 10; // y is mutable
    println!("The value of y is: {}", y);
    y = 15; // This is allowed because y is mutable
    println!("The new value of y is: {}", y);
}

fn shadowing() {
    let x = 5;
    let x = x + 1; // x is shadowed here
    {
        let x = x * 2; // x is shadowed again in this inner scope
        println!("The value of x in the inner scope is: {}", x); // 12
    }
    println!("The value of x in the outer scope is: {}", x); // 6
}

//Type of data

fn scalar_types() {
    let integer: i32 = 42; // 32-bit signed integer
    let float: f64 = 3.14; // 64-bit floating point
    let boolean: bool = true; // Boolean type
    let character: char = 'R'; // Character type

    println!("Integer: {}, Float: {}, Boolean: {}, Character: {}", integer, float, boolean, character);
}
