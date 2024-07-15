use std::io;
use std::fs;

fn main() {
    let arg = String::from("-");

    // We need to describe the type to get dynamic dispatch.
    let readable: &mut dyn io::Read = if arg == "-" {
        &mut io::stdin()
    } else {
        &mut fs::File::open(arg)?
    };

    // Read from `readable` here.
}