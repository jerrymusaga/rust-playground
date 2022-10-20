#[derive(Debug)]
struct User {
    name: String,
    age: u32,
    active: bool
}

fn set_user() -> User {
    User {
        name: String::from("Henry"),
        age: 45,
        active: false
    }
}

fn main() {
    let james = User {
        name: String::from("James"),
        age: 32,
        active: true
    };
    println!("{:?}", james);

    let henry = set_user();
    println!("{:?}", henry);
}
