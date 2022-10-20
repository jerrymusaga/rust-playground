#[derive(Debug)]
struct User {
    name: String,
    age: u32,
    active: bool
}

impl User {
    fn new(name: String, age: u32, active: bool) -> Self {
        Self {
            name,
            age,
            active
        }
    }
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

    let emma = User::new("emma".to_string(), 22, false);
    let juliet = User::new(String::from("juliet"), 25, true);

    println!("{:?}->{:?}", emma, juliet);
}
