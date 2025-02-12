use std::io;

fn main() {
    loop {
        let mut given_fah = String::new();

        io::stdin()
            .read_line(&mut given_fah)
            .expect("Failed to read line");
        
        let given_fah: i32 = match given_fah.trim().parse() {
            Ok(num) => num,
            Err(_)  => {
                println!("Please enter an integer");
                continue;
            }
        };

        println!("Given Fahrenhiet is: {given_fah}");
        // let given_fah: f32 = given_fah as f32;
        let celcius = { given_fah as f32 - 32.0 } * { 5.0/9.0 };

        println!("{given_fah} °F is {celcius} °C");
    }
}