use crate::webservice_aerolineas;
use crate::webservice_hoteles;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::{thread, time::Duration};

pub struct Reserve {
    origin: String,
    destination: String,
    airline: String,
    hotel: String
}

const DELAY_BETWEEN_RETRIES: u64 = 5;

pub fn reserve_airline(origin: &str, destination: &str){
    let approved: bool = webservice_aerolineas::reservar(origin.to_string(), destination.to_string());
    if !approved {
        println!("La aerolinea no aprobó la reserva. Reintentando en {} segundos", DELAY_BETWEEN_RETRIES);
        thread::sleep(Duration::from_millis(DELAY_BETWEEN_RETRIES*1000));
        println!("Reintentando...");
        reserve_airline(origin, destination);
        return;
    }
    println!("La aerolinea aprobó la reserva con origen: {} y destino: {}", origin, destination);
}

pub fn reserve_hotel(hotel: &str){
    webservice_hoteles::reservar(hotel.to_string());
    println!("El servicio de hoteles aprobó la reserva en: {}", hotel);
}

pub fn process_reserve(reserve: Reserve){
    println!("A new thread is reading the reserve with Airline {} and Hotel {}", reserve.airline, reserve.hotel);
    reserve_airline(&reserve.origin, &reserve.destination);
    reserve_hotel(&reserve.hotel);
}

pub fn parse_reserves(filename: &str){
    // Make a vector to hold the children which are spawned.
    let mut children = vec![];
    if let Ok(lines) = read_lines(filename) {
        // Consumes the iterator, returns an (Optional) String
        for reserve_line in lines.into_iter().flatten() {
            let reserve_split: Vec<&str> = reserve_line.split(' ').collect();
            let reserve = Reserve{origin: reserve_split[0].to_string(),
                destination: reserve_split[1].to_string(),
                airline: reserve_split[2].to_string(),
                hotel: reserve_split[3].to_string()};
            children.push(thread::spawn(move || process_reserve(reserve)));
        }
    }

    for child in children {
        // Wait for the thread to finish. Returns a result.
        let _ = child.join();
    }
    println!("Reserve processing finished");
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
