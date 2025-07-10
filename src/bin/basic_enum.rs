// Topic: Working with an enum
//
// Program requirements:
// * Print the name of color to the terminal
//
// Notes:
// * Use an enum with coloer names as variants
// * Use a function to print the color name
// * The function must use the enum as a parameter
// * Use a match expression to determine which color name to print


enum Colors {
    Red,
    Blue,
    Green
}

fn print_color(my_color: Colors) {
    match my_color {
        Colors::Red => println!("red"),
        Colors::Blue => println!("blue"),
        Colors::Green => println!("greed")
    }
}


fn main() {
    print_color(Colors::Blue);
    print_color(Colors::Green);
    print_color(Colors::Red);
}
