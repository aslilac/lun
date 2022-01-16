use std::{fs, io};

fn main() -> io::Result<()> {
    let _ = fs::copy("./a", "./b")?;
    println!("Hi at build time!");

    println!("cargo:rerun-if-changed=a");

    Ok(())
}

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug, PartialEq)]
enum OperandAbbrv {
    i,
    r,
    x,
}

trait Operand {
    const ABBR: OperandAbbrv;
}
