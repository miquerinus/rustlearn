use std::io;
use std::io::prelude::*;

fn main() {

    println!("Programming in rust language.\n");
    print!("Tecle <enter> para encerrar ... ");
    io::stdout().flush().unwrap();
    io::stdin().read(&mut[0u8]).unwrap();

}
