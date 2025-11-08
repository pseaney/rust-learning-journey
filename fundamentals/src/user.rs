use crate::status::Status;

pub struct User {
    pub username: String,
    pub age: u32,
    pub status: Status,
}

impl User {
    pub fn new(username: &str, age: u32) -> Self {
        User {
            username: username.to_string(),
            age,
            status: Status::Pending,
        }
    }

    pub fn greet(&self) {
        println!("Hello, {}!", self.username);
        self.status.print();
    }

    pub fn activate(&mut self) {
        self.status = Status::Active;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::status::Status;

    #[test]
    fn test_user_creation() {
        let user = User::new("Paul", 30);
        assert_eq!(user.username, "Paul");
        assert_eq!(user.age, 30);
        matches!(user.status, Status::Pending);
    }

    #[test]
    fn test_user_activation() {
        let mut user = User::new("Paul", 30);
        user.activate();
        matches!(user.status, Status::Active);
    }
}