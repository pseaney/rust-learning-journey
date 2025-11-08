pub trait Printable {
    fn print(&self);
}

pub struct Article {
    pub title: String,
}

pub struct Comment {
    pub content: String,
}

impl Printable for Article {
    fn print(&self) {
        println!("Article: {}", self.title);
    }
}

impl Printable for Comment {
    fn print(&self) {
        println!("Comment: {}", self.content);
    }
}

// Generic function with trait bound
fn show<T: Printable>(item: T) {
    item.print();
}

fn main() {
    let a = Article { title: "Rust Traits".into() };
    let c = Comment { content: "Nice post!".into() };

    show(a);
    show(c);
}