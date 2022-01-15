use std::{env, fs, io};

fn main() -> Result<(), io::Error> {
    let args = env::args();

    for arg in args.skip(1) {
        let content = fs::read_to_string(arg)?;
        println!("{}", content);
    }

    Ok(())
}
