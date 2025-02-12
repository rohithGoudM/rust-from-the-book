fn main() {
    let days:    [&str; 12] = ["first", "second", "third", "fourth", "fifth", "sixth", 
        "seventh", "eighth", "ninth", "tenth", "eleventh", "twelfth"];
    let mut numbers:  [&str; 12] = ["A", "Two", "Three", "Four", "Five", "Six", 
        "Seven", "Eight", "Nine", "Ten", "Eleven", "Twelve"];
    let gifts:    [&str; 12] = [
        "partridge in a pear tree",
        "turtle doves",
        "French hens",
        "calling birds",
        "golden rings",
        "geese a-laying",
        "swans a-swimming",
        "maids a-milking",
        "ladies dancing",
        "lords a-leaping",
        "pipers piping",
        "drummers drumming",
    ];

    let mut index = 0;
    while index < 12 {
        let day    = days[index];
        println!("On the {day} day of Christmas,");
        println!("my true love gave to me");
        for i in (0..index+1).rev() {
            let number = numbers[i];
            let gift   = gifts[i];
            print!("{number} {gift}");
            if i == 0 {
                println!(".");
            } else {
                println!(",");
            }
        }
        if index == 0 {
            numbers[0] = "And a";
        }
        println!("");
        index += 1;
    }
}
