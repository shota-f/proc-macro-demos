use simple_attr::add_messages;

#[add_messages]
fn hey() {
    println!("Some processes...");
}

fn main() {
    hey()
}
