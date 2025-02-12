fn main() {

    // Rust allows you to shadow variables
    let x = 5;
    let x = x + 1;
    {
        let x = x * 2;
        println!("The value of x in the inner scope is {x}");
    }
    println!("The value of x in the outer scope is {x}");

    let mut x = 5;
    println!("The value of x is {x}");
    x = 6;
    println!("The value of x is {x}");

    //type of the value must be annotated for const
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

    let spaces = "     ";
    let spaces = spaces.len();

    println!("Length of spaces is {spaces}");

    // let mut spaces = "    ";
    // spaces = spaces.len() // throws error
}
