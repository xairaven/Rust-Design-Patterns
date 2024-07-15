use std::io;
use std::fs;

fn main() {
    let arg = String::from("-");

    // We need to describe the type to get dynamic dispatch.
    let readable: Box<dyn io::Read> = if arg == "-" {
        Box::new(io::stdin())
    } else {
        Box::new(fs::File::open(arg)?)
    };

    // Read from `readable` here.
}