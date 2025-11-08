pub enum Status {
    Active,
    Inactive,
    Pending,
}

impl Status {
    pub fn print(&self) {
        match self {
            Status::Active => println!("Status: Active"),
            Status::Inactive => println!("Status: Inactive"),
            Status::Pending => println!("Status: Pending"),
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_status_print() {
        let status = Status::Inactive;
        status.print(); // Just ensures it doesn't panic
    }
}