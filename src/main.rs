// fn main() {
//     let mut name = "Rob"; //Rust non owned String type, immutable by default. If you want to own the memory i.e. need to return a String from a function then use String instead.
//     println!("Hello, {}!", name);
//     name = "Bill";              // if you don't make 'name' mutable then you can't re-assign it to something else later
//                                 // In fact the vscode editor will actually point this out - probably Rust Analyzer doing this!
//     println!("hello, {}!", name);
// }

fn show(x: &str) {
    println!("Hi, {}", x)
}

fn main() {
    let name = "Rob";
    show(name);
}
