mod printable;
mod article;
mod comment;

use printable::Printable;
use article::Article;
use comment::Comment;

fn main() {
    let items: Vec<Box<dyn Printable>> = vec![
        Box::new(Article { title: "Rust Traits".to_string() }),
        Box::new(Comment { content: "Great post!".to_string() }),
    ];

    for item in items {
        item.print();
    }
}