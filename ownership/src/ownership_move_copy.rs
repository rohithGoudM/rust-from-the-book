fn main() {
    // Scope
    // A range within a program in which an item is valid.
    // Rust drops a variable/item when it goes out of scope.

    // The String Type
    let mut s = String::from("hello");
    s.push_str(", world");
    println!("{s}");

    // variable assignment
    let x = 5;
    let y = x;
    // 5 is a scalar data type (int) which is pushed on to the stack and is assigned to x
    // then 5 is copied/duplicated and pushed and assigned to y

    let s1 = String::from("new");
    let s2 = s1;
    // A new string is allocated to heap and its pointer along with len and capacity are stored in stack as s1.
    // Then the s1 is copied to s2 and s1 is invalidated. Cannot use s1 again.
    // In short, s1 is moved to s2 (not copied)

    let mut s = String::from("something_old");
    s = String::from("something_new");
    // here the inverse is true. the 'something_old' is dropped right when it was replaced by a new value.

    let s1 = String::from("hello");
    let s2 = s1.clone();
    // this will do a deep clone, even the heap data is copied onto s2.

    // Now remeber, this works fine without any fuss
    let x = 5;
    let y = x;
    // No invalidation is going on here.
    // Primitive Scalar types are copied not moved
    // The reason is that types such as integers that have a known size at compile time are stored entirely on the stack, 
    // so copies of the actual values are quick to make.

    // Copy is a trait
    // other data types that implement Copy
    // integers: ex - u32
    // boolean
    // floating points
    // char
    // derived structs from above types
    // such as tuples with (u32, bool) not (u32, String)

    // Functions
    let s = String::from("hello");  // s comes into scope

    takes_ownership(s);             // s's value moves into the function...
                                    // ... and so is no longer valid here

    let x = 5;                      // x comes into scope

    makes_copy(x);                  // x would move into the function,
                                    // but i32 is Copy, so it's okay to still
                                    // use x afterward

} // Here, x goes out of scope, then s. But because s's value was moved, nothing
  // special happens.

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{some_string}");
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{some_integer}");
} // Here, some_integer goes out of scope. Nothing special happens.