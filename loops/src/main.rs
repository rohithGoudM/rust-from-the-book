fn return_from_loop() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
        println!("{}", counter);
    };

    println!("The result is {result}");

}

fn loop_labelling() {
    let mut count = 0;
    let out_loop_num = 'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 0 {
                break;
            }
            if count == 2 {
                break 'counting_up count * 4;
            }
            remaining -= 1;
        }
        count += 1;
    };
    println!("End count = {out_loop_num}");
}

fn while_loop() {
    let mut number = 3;

    while number > 0 {
        println!("{number}");
        number -= 1;
    }
    println!("LIFT OFF!!!");
}

fn for_loop() {
    let a = [1,2,3,4,5];

    for element in a {
        println!("the value is: {element}");
    }
}

fn main() {
    for number in (1..4).rev() {
        println!("{number}");
    }
    println!("LIFT OFF!!!");
}