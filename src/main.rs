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
    let y: &str = x.trim();             // trim removes whitespace and carriage return characters off the input
    println!("Hi {}, {}", y, PROPS);
}

fn main() {

    println!("Hello, what's your name?");
    
    let mut name: String = String::new();

    io::stdin().read_line(&mut name)
        .expect("Failed to read line");

    show(& name);
}

/*
Shadowing - allows you to re-use variable names AS LONG AS you don't change their type
            basically doing transitions on a value but having that value be immutable after those transitions have been done
            you're effectively creating a new variable when you use the let keyword again
*/