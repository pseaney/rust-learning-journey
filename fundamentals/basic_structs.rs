struct User {
    username: String,
    age: u32,
    active: bool,
}

impl User {
    fn new(username: &str, age: u32) -> Self {
        User {
            username: username.to_string(),
            age,
            active: true,
        }
    }

    fn greet(&self) {
        println!("Hello, {}!", self.username);
    }

    fn deactivate(&mut self) {
        self.active = false;
    }
}

fn main() {
    let mut user = User::new("Paul", 30);
    user.greet();
    user.deactivate();
    println!("User active: {}", user.active);
}