use std::fs;

fn main() {
    let _ = fs::copy("./a", "./b");
    println!("Hi at build time!");
}
