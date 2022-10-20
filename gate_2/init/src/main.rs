#[derive(Debug)]
struct User {
    name: String,
    age: u32,
    active: bool
}


fn main() {
    let james = User {
        name: String::from("James"),
        age: 32,
        active: true
    };
    println!("{:?}", james);
}
