use crate::printable::Printable;

pub struct Article {
    pub title: String,
}

impl Printable for Article {
    fn print(&self) {
        println!("Article: {}", self.title);
    }
}