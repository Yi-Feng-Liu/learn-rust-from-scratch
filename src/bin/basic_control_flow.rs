// Topic: Flow control using if...else
//
// Program requirements:
// * Display s message based on the value of boolean variable
// * When the variable is set to true, display "hi"
// * When the variable is set to false, display "bye"
// * Use a if...else if...else to perform the flow control


// this is the other practice
fn buy_something() {
    let age = 15;
    if age >= 21 {
        println!("OK to purchase")
    } else {
        println!("Cannot  purchase")
    }
}


// if...else if block
fn age_verify(age: i32) {
    if age >= 21 {
        println!("OK to purchase");
    } else if age >= 18 {
        println!("You need to show the ID to verify your age")
    } else {
        println!("Cannot purchase")
    }
}


fn main(){
    buy_something();
    let my_bool = true;
    if my_bool == true {
        println!("hi");
    } else{
        println!("bye");
    }

    age_verify(21);

}
