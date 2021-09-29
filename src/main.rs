use std::env;

mod reserve_module {
    use std::fs::File;
    use std::io::{self, BufRead};
    use std::path::Path;
    use std::thread;
    
    pub struct Reserve {
        airline: String,
        hotel: String
    }

    pub fn parse_reserves(filename: &String){
        // Make a vector to hold the children which are spawned.
        let mut children = vec![];
        if let Ok(lines) = read_lines(filename) {
            // Consumes the iterator, returns an (Optional) String
            for line in lines {
                if let Ok(reserve_line) = line {
                    let reserve_split: Vec<&str> = reserve_line.split(" ").collect();
                    let reserve = Reserve{airline: reserve_split[0].to_string(), hotel: reserve_split[1].to_string()};
                    children.push(thread::spawn(move || {
                        println!("A new thread is reading the reserve with Airline {} and Hotel {}", reserve.airline, reserve.hotel);
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
}
fn main() {

    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    reserve_module::parse_reserves(filename)
}