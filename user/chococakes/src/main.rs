use hello_macro_2::HelloMacro2;
use hello_macro_derive_2::HelloMacro2;

#[derive(HelloMacro2)]
struct ChokoCakes {
    choco_type: String
}

fn main() {
    ChokoCakes::hello_macro_2();
}
