mod printable;
mod article;
mod comment;

use printable::Printable;
use article::Article;
use comment::Comment;

fn create_item(kind: &str) -> Box<dyn Printable> {
    match kind {
        "article" => Box::new(Article { title: "Rust Traits".into() }),
        "comment" => Box::new(Comment { content: "Nice post!".into() }),
        _ => panic!("Unknown kind"),
    }
}

fn main() {
    let item1 = create_item("article");
    let item2 = create_item("comment");

    item1.print();
    item2.print();
}