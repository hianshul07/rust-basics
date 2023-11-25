// * Varibales and mutability
// Variables are immutable by default
// we can make variables mutable by using 'mut' keyword

// * Constants
// The const varibale name must be written in UPPER_SNAKE_CASE
// constants are immutable by defualt
// it is necessary to mention type of const value
// The constants can be declared in the global scope

// * Variable shadowing
// when you re-declare a variable with 'let' keyword, then it is called shadowing
// the compiler will read the value which is declared later until its scope

const HEHE: &str = "hehehehe";

fn main() {
    // immutability of variables
    let mut x = 5;
    println!("{}", x);
    x = 12;
    println!("{}\n", x);

    // const value
    println!("{}\n", HEHE);

    // shadowing
    let y = 8;
    println!("{}", y);

    let y = 16;
    println!("{}", y)
}
