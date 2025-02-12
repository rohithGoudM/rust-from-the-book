use std::io;

fn main() {
    let mut n = String::new();

    io::stdin()
        .read_line(&mut n)
        .expect("Enter an integer");
    
    let n: i32 = match n.trim().parse() {
        Ok(num) => num,
        Err(_)  => {
            println!("Please enter a valid number");
            5
        }
    };
    
    println!("The value of n is {n}");

    match n {
        0 => println!("Please enter numbers greater than 0."),
        1 => println!("The first fibonacci number is 0"),
        2 => println!("The second fibonacci number is 1"),
        i32::MIN..=-1_i32 | 3_i32..=i32::MAX => {
            let mut fib1 = 0;
            let mut fib2 = 1;
            let mut fib_cur = 1;
            let mut count = 3;

            while count <= n {
                fib_cur = fib1 + fib2;
                fib1 = fib2;
                fib2 = fib_cur;
                count += 1;    
            }

            println!("The nth fibonacci number given n={n} is {fib_cur}.");
        }
    }
}