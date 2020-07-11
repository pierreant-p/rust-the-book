struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

// tuple Struct
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    let user1 = User {
        username: String::from("my-username"),
        email: String::from("someone@example.com"),
        sign_in_count: 12,
        active: true,
    };

    let username = String::from("user2");
    let email = String::from("user2@example.com");
    let user2 = build_user(username, email);
    print_user(&user2);

    let user3 = User {
        email: String::from("user3@example.com"),
        username: String::from("user3"),
        ..user1
    }; // user values from user1
    print_user(&user3);

    // tuple structs
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    println!("{}", black.1);
    println!("{}", origin.1);
}

fn build_user(username: String, email: String) -> User {
    // User {
    //     username: username,
    //     email: email,
    //     active: true,
    //     sign_in_count: 1,
    // }

    // This is equivalent to above
    User {
        username,
        email,
        active: true,
        sign_in_count: 1,
    }
}

fn print_user(user: &User) {
    println!("{}", user.username);
    println!("{}", user.email);
    println!("{}", user.sign_in_count);
    println!("{}", user.active);
}
