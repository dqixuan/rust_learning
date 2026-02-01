
fn main() {

    let mut user1 = User {
        active: true,
        username: String::from("some user name 123"),
        email: String::from("some@example.com"),
        sign_in_count: 1,
    };
    println!("email of user1 is {}", user1.email);
    user1.email = String::from("deyi");
    println!("email of user1 is {}", user1.email);

    // struct update syntax, 
    // .. specify the remaining fields have the same value as the fields in the given instance.
    let mut user2 = User {
        email: String::from("111"),
        ..user1
    };

    let black = Color(0, 0, 0);

    let Point(x, y, z) = black;
}

// keyword struct + name, get field value by dot notation
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

// field init shorthand, only when parameter name is same as the field name.
fn build_user(username: String, email: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 3,
    }
}

// tuple structs
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

// Unit-like structs
struct AlwaysEqual;




struct Rectangle {
    width: f32,
    height: f32,
}

// method of Rectangle
impl Rectangle {
    fn area(self) -> f32 {
        width * height
    }
}
