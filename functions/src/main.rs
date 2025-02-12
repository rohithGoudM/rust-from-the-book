fn main() {
    println!("Hello, world!");
    another_funtion();
    function_with_parameters(9);
    print_labeled_measurement(3, 'A');
    let y = plus_one(4);
    println!("The value of y is {y}");
    // let y = plus_one_with_semicolon(8);
}

fn another_funtion() {
    println!("Another function");
}

fn function_with_parameters(x: i32) {
    println!("The value of x is {x}")
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

// fn plus_one_with_semicolon(x: i32) -> i32 {
//     x + 1;
// }