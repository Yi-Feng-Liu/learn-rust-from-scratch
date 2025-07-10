// Topic: Looping using loop statement and while loop statement
//
// Program requirements:
// * Create a loop that prints numbers from 1 to 10.
// * Display "1" through "4" in the terminal
// * Display 10 to 1 using while loop
//
// Notes:
// * Use a mutable integer variable
// * Use a loop statement
// * Print the variable within the loop statement
// * Use break to exit the loop


// Practice 1 -> Looping with counter
fn loop_counter() {
    let mut num = 1;
    loop {
        if num <= 4 {
            println!("{:?}", num);
        }
        println!("Loop {num}");
        if num == 10 {
            break;
        }
        num += 1;
    }
}

// Practice 2 -> Looping with while
fn while_counter() {
    let mut num = 10;
    while num >= 1 {
        println!("{:?}", num);
        num -= 1;
    }
    println!("Done !");
}


fn main() {
    loop_counter();
    while_counter();
}
