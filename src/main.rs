use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::thread;

fn main() {

    // Make a vector to hold the children which are spawned.
    let mut children = vec![];

    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    if let Ok(lines) = read_lines(filename) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(reserva) = line {
                children.push(thread::spawn(move || {
                    println!("A new threas is reading the line {}", reserva);
                }));
            }
        }
    }
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}