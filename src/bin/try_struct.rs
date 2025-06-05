struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    let user_1 = User {
        email: String::from("example_email@hotmail.com"),
        active: true,
        username: String::from("John Smith"),
        sign_in_count: 0,
    };

    let user_2 = User {
        email: String::from("user2email@email.com"),
        ..user_1
    };

    // The ownership of username moved from user_1 to user_2.
    // Access username data via user_1 will cause an error.
    // Fields like active and sign_in_count are stack memory type,
    // which are copied and no ownership movement issues.

    // println!("{}", user_1.username);
}

fn print_user(user: &User) {
    println!("Username: {}", user.username);
    println!("Email: {}", user.email);
    println!("Is active: {}", user.active);
    println!("Has signed in {} times.", user.sign_in_count);
}