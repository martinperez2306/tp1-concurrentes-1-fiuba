use std::io::Write;
use std::fs::OpenOptions;

pub fn log(message: String){
    let mut file = OpenOptions::new().write(true).create(true).append(true).open("resources/reserves.log").unwrap();
    println!("{}", message);
    if let Err(e) = writeln!(file, "{}", message){
        println!("Couldn't write to file: {}", e);
    };
}