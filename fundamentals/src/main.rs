mod user;
mod status;

use user::User;

fn main() {
    let mut user = User::new("Paul", 30);
    user.greet();
    user.activate();
    user.greet();
}