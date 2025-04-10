struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}
fn main() {
    let user1 = User {
        active: true,
        email: String::from("example@example.com"),
        username: String::from("user1"),
        sign_in_count: 1,
    };
}
