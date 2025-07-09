// Topic: Print a message to the terminal
//
// Programe requirements:
// * Distplay a message to the terminal and use (a) variable(s) and a function
//
// Notes:
// * Use a function to display the message
// * Use a variable to store the message


fn print_line() {
    println!("Hello, Rust!");
    // method of print variable

    // The method 1 usually used for debugging that will be like: Hello, "Rust!"
    // It is difference from the other method
    println!("Hello, {:?}", "Rust!");

    // The method 2 is more readable
    // It is more suitable for user interface
    println!("Hello, {}", "Rust!");

    // if you want to print a variable, you can use the following method
    let message= "Hello, Rust!";
    println!("{}", message);
}

// To run this program, you can use the following command below:
// cargo run -q --bin print
fn main() {
    print_line();
}