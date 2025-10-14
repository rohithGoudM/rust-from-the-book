struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn build_user (email: String, username: String) -> User {
    User {
        active: true,
        username: username,
        email: email,
        sign_in_count: 1
    }
}

fn main() {
    let mut user1 = User {
        active: true,
        email: String::from("example@example.com"),
        username: String::from("user1"),
        sign_in_count: 1,
    };
    
    user1.email = String::from("anotheremail@example.com");

    println!("{}", user1.email);
    
    let user2 = build_user(String::from("user2@example.com"), String::from("naamMeraUser2"));

    println!("{}", user2.username);
    
}
