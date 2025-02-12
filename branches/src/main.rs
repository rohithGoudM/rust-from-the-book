fn main() {
    let number = 7;

    if number < 5 {
    // if number {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    let number = 3;

    if number == 0 {
        println!("number was zero");
    } else if number != 1 {
        println!("number isn't 1 either");
    }

    let condition = true;

    let number = if condition { 5 } else { 6 };

    println!("The value of the number is {number}");
}