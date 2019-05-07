use simple_attr::{add_messages, add_messages_2};

#[add_messages]
fn func() {
    println!("func: Some processes...");
}

#[add_messages_2("Hello, I am func_2")]
fn func_2() {
    println!("func_2: Some processes...");
}

fn main() {
    func();

    func_2();
}
