// define Book struct
struct Book {
    title: String,
    author: String,
    pages: u32,
    available: bool,
}

// define User struct
struct User{
    username: String,
    email: String,
    active: bool,
    sign_in_count: u32,
}

// tuple structs
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

// Unit-Like Struct
struct AlwaysEqual;

// defining a function to make it easier to use
fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

fn build_book(title: String, author: String, pages: u32, available: bool) -> Book {
    Book {
        title,
        author,
        pages,
        available,
    }
}

fn main() {
    // Creating objects
    let mut book = build_book(String::from("Rust"), String::from("The Book"), 1000, true);
    let mut user1 = build_user(String::from("user1@gmail.com"), String::from("user1"));
    let mut black = Color(0, 0, 0);
    let mut red = Color(255, 0, 0);
    let subject = AlwaysEqual;
    // modify values
    user1.email = String::from("user@hotmail.com");
    println!("user1 email: {}", user1.email);
}
