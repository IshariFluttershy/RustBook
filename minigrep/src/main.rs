use std::env;
 
//use crate::lib;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let config = mylib::Config::new(&args);

    mylib::run(&config).unwrap();
    
    println!("");
    Ok(())
}


