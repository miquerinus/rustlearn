use std::io;
use std::prelude::*;

fn main() {

    println!("**** Programming in rust language.... welcome.******\n");
    print!("Press <enter> to exit ... ");
    io::stdout().flush().unwrap();
    io::stdin().read(&mut[0u8]).unwrap();

}
