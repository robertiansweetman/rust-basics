// fn main() {
//     let mut name = "Rob"; //Rust non owned String type, immutable by default. If you want to own the memory i.e. need to return a String from a function then use String instead.
//     println!("Hello, {}!", name);
//     name = "Bill";              // if you don't make 'name' mutable then you can't re-assign it to something else later
//                                 // In fact the vscode editor will actually point this out - probably Rust Analyzer doing this!
//     println!("hello, {}!", name);
// }

use std::io;

const PROPS: &str = "you're AMAZING!";

fn show(x: &str) {
    let y: &str = x.trim();
    println!("Hi {}, {}", y, PROPS);
}

fn main() {

    println!("Hello, what's your name?");
    
    let mut name: String = String::new();

    io::stdin().read_line(&mut name)
        .expect("Failed to read line");

    show(& name);
}
