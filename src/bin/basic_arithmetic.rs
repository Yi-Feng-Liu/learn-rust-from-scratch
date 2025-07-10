// Topic: Basic Arithmetic Operations
//
// Program requirements:
// * Display the result of two numbers together
//
// Note:
// * Use a function to perform the addition of two numbers



fn sum(a: i32, b:i32) -> i32 {
    a + b
}

fn display_result(result: i32) {
    println!("{:?}", result);
}

fn main() {
    // let diff = 10 - 5;
    // let div = 10 / 2;
    // let mult = 5 * 6;
    // let rem = 6 % 3;
    // let rem2 = 6 % 4;

    let result = sum(8, 3);
    display_result(result);

}
