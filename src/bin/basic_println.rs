// * Topic: Functions
//
// * Program requirements:
// Display a simple message in the terminal
//
// * Notes:
// Use a function to display the message and used different methods to display variables in the terminal


fn basic_println() {
    println!("Hello, World");
    // method 1
    println!("Hello, {}!", "Rustaceans");
    // method 2
    println!("Hello, {name}!", name = "Rustaceans");
    // method 3 usually used to debug
    println!("This methid is usually used to {:?}", "Debuging!");
}

fn main () {
    basic_println();
}
