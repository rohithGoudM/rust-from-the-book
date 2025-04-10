fn main() {
    // let s1 = String::from("hello");

    // let len = calculate_length(&s1);

    // println!("The length of '{s1}' is {len}.");


    let s = String::from("hello");

    change(&s);
}

fn change(some_string: &String) {
    some_string.push_str(", world");
}

fn calculate_length(s: &String) -> usize {
    s.len()
} 