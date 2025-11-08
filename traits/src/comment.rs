use crate::printable::Printable;

pub struct Comment {
    pub content: String,
}

impl Printable for Comment {
    fn print(&self) {
        println!("Comment: {}", self.content);
    }
}