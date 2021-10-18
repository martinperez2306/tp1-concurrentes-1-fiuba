mod model;
extern crate std_semaphore;

use crate::reserve_controller::model::route::Route;
use crate::webservice_aerolineas;
use crate::webservice_hoteles;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, SystemTime};
use std_semaphore::Semaphore;
use crate::reserve_controller::model::flight::Flight;
use crate::reserve_controller::model::logger;
use crate::reserve_controller::model::package::Package;
use crate::reserve_controller::model::stats::Stats;

const NO_HOTEL: &str = "-";
const DELAY_BETWEEN_RETRIES_SECONDS: u64 = 5;
const WEBSERVICE_AIRLINE_LIMIT: isize = 10;
const WEBSERVICE_HOTEL_LIMIT: isize = 5;
const STATS_LOG_PERIOD: u64 = 1;

pub fn reserve_airline(origin: &str, destination: &str, airline: &str, airline_sem: &Arc<Semaphore>){
    logger::log(format!("Reservando aerolinea {}", airline));
    airline_sem.acquire();
    let approved: bool = webservice_aerolineas::reservar(origin.to_string(), destination.to_string());
    airline_sem.release();
    if !approved {
        logger::log(format!("La aerolinea no aprobó la reserva. Reintentando en {} segundos", DELAY_BETWEEN_RETRIES_SECONDS));
        thread::sleep(Duration::from_millis(DELAY_BETWEEN_RETRIES_SECONDS*1000));
        reserve_airline(origin, destination, airline, airline_sem);
        return;
    }
    logger::log(format!("La aerolinea aprobó la reserva con origen: {} y destino: {}", origin, destination));

}

pub fn reserve_hotel(hotel: &str, hotel_sem: &Arc<Semaphore>) {
    hotel_sem.access();
    webservice_hoteles::reservar(hotel.to_string());
    logger::log(format!("El servicio de hoteles aprobó la reserva en: {}", hotel));
}

pub fn process_flight(flight: &Flight, airline_sem: Arc<Semaphore>){
    let initial_process_time = SystemTime::now();
    let origin = flight.get_origin();
    let destination = flight.get_destination();
    let airline = flight.get_airline();
    let _ = thread::spawn(move || reserve_airline(&origin, &destination, &airline, &airline_sem)).join();
    let final_process_time = SystemTime::now();
    let difference = final_process_time.duration_since(initial_process_time)
    .expect("Clock may have gone backwards");
    println!("La reserva de vuelo se proceso en {:?} segundo(s)", difference);
}

pub fn process_package(package: &Package, airline_sem: Arc<Semaphore>, hotel_sem: Arc<Semaphore>){
    let initial_process_time = SystemTime::now();
    let mut children = vec![];
    let origin = package.get_origin();
    let destination = package.get_destination();
    let airline = package.get_airline();
    let hotel = package.get_hotel();
    children.push(thread::spawn(move || reserve_airline(&origin, &destination, &airline, &airline_sem)));
    children.push(thread::spawn(move || reserve_hotel(&hotel, &hotel_sem)));
    for child in children {
        let _ = child.join();
    }
    let final_process_time = SystemTime::now();
    let difference = final_process_time.duration_since(initial_process_time)
    .expect("Clock may have gone backwards");
    println!("La reserva de paquete se proceso en {:?} segundo(s)", difference);
}

pub fn logs_stats(log_stats_mutex: Arc<Mutex<bool>>, stats_mutex: Arc<Mutex<Stats>>){
    let mut processing = true;
    while processing {
        let stats_block = stats_mutex.lock().unwrap();
        println!("{:?}", stats_block.get_routes());
        drop(stats_block);
        thread::sleep(Duration::from_millis(STATS_LOG_PERIOD*1000));
        let log_stats_lock = log_stats_mutex.lock().unwrap();
        processing = *log_stats_lock;
        drop(log_stats_lock);
    }
}

pub fn increment_stats(stat_mutex: Arc<Mutex<Stats>>, route: Route){
    let mut stats_block = stat_mutex.lock().unwrap();
    stats_block.increment_route_counter(route);
}

pub fn parse_reserves(filename: &str){
    let parsing_reserves = true;
    // Make a vector to hold the children which are spawned.
    let mut children = vec![];
    let airline_sem = Arc::new(Semaphore::new(WEBSERVICE_AIRLINE_LIMIT));
    let hotel_sem = Arc::new(Semaphore::new(WEBSERVICE_HOTEL_LIMIT));
    let stats: Stats = Stats::new();
    let stat_mutex = Arc::new(Mutex::new(stats));
    let stat_mutex_clone = stat_mutex.clone();
    let log_stats_mutex = Arc::new(Mutex::new(parsing_reserves));
    let log_stats_mutex_clone = log_stats_mutex.clone();
    let stats_thread = thread::spawn(move || logs_stats(log_stats_mutex, stat_mutex));
    if let Ok(lines) = read_lines(filename) {
        // Consumes the iterator, returns an (Optional) String
        for reserve_line in lines.into_iter().flatten() {
            let reserve_split: Vec<&str> = reserve_line.split(' ').collect();
            let origin = reserve_split[0].to_string();
            let destination = reserve_split[1].to_string();
            let airline = reserve_split[2].to_string();
            let hotel = reserve_split[3].to_string();
            let airline_sem_clone = airline_sem.clone();
            let route = Route::new(origin.clone(), destination.clone());
            let stat_mutex_clone_it = stat_mutex_clone.clone();
            if hotel == NO_HOTEL {
                children.push(thread::spawn(move || process_flight(&Flight::new(origin, destination, airline), airline_sem_clone)));
            } else {
                let hotel_sem_clone = hotel_sem.clone();
                children.push(thread::spawn(move || process_package(&Package::new(origin, destination, airline, hotel), airline_sem_clone, hotel_sem_clone)));
            }
            children.push(thread::spawn(move || increment_stats(stat_mutex_clone_it, route)));
        }
        println!("Esperando a que termine el procesamiento de Reservas");
        for child in children {
            // Wait for the thread to finish. Returns a result.
            let _ = child.join();
        }
        let mut log_stats_lock = log_stats_mutex_clone.lock().unwrap();
        *log_stats_lock = false;
        drop(log_stats_lock);
        let _stats_thread_join = stats_thread.join();
        println!("Procesamiento de Reservas terminado");
    }
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
