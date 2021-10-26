extern crate std_semaphore;

use crate::model::airlines_semaphore::AirlinesSemaphore;
use crate::model::flight::Flight;
use crate::model::logger;
use crate::model::package::Package;
use crate::model::route::Route;
use crate::model::stats::Stats;
use crate::webservice::webservice_aerolineas;
use crate::webservice::webservice_hoteles;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::sync::mpsc;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, SystemTime};
use std_semaphore::Semaphore;

const NO_HOTEL: &str = "-";
const DELAY_BETWEEN_RETRIES_SECONDS: u64 = 5;
const WEBSERVICE_HOTEL_LIMIT: isize = 5;
const STATS_LOG_PERIOD: u64 = 3;

/**
 * Process all the reserves stored in a specific file.
 * Recieves file system path
 */
pub fn process_reserves(filename: String) {
    logger::log(format!("Procesamiento de Reservas iniciado"));
    let (processing_reserves_tx, processing_reserves_rx) = mpsc::channel();
    if let Err(error) = processing_reserves_tx.send(true){
        println!("Ocurrio un error enviando a traves del channel: {}", error);
    }
    let mut processing_steps = vec![];
    let stats: Stats = Stats::new();
    let stats_mutex = Arc::new(Mutex::new(stats));
    let stats_mutex_for_parse = stats_mutex.clone();
    let stats_mutex_for_log = stats_mutex.clone();
    let parse_reserves_thread = thread::spawn(move || {
        parse_reserves(processing_reserves_tx, &filename, stats_mutex_for_parse)
    });
    let log_stats_thread =
        thread::spawn(move || logs_stats(processing_reserves_rx, stats_mutex_for_log));
    processing_steps.push(parse_reserves_thread);
    processing_steps.push(log_stats_thread);
    for step in processing_steps {
        let _ = step.join();
    }
    let stats_lock_result = stats_mutex.lock();
    match stats_lock_result {
        Ok(stats_block) => {
            let avg_reserve_processing_time = stats_block.get_avg_reserve_processing_time();
            println!(
                "El tiempo medio de procesamiento de una reserva es {} segundos",
                avg_reserve_processing_time
            );
            logger::log(format!("Procesamiento de Reservas terminado"));
        }
        Err(e) => {
            println!("Algo salió mal con el stats_lock. Error: {}", e);
        }
    }
}

/**
 * Registers the statistics of the 10 most requested routes periodically until the system finishes processing the total reserves
 */
pub fn logs_stats(processing_reserves_rx: Receiver<bool>, stats_mutex: Arc<Mutex<Stats>>) {
    let mut processing = true;
    while processing {
        let stats_lock_result = stats_mutex.lock();
        match stats_lock_result {
            Ok(stats_block) => {
                println!("---------------LAS 10 RUTAS MAS SOLICITADAS---------------");
                let routes: HashMap<String, u32> = stats_block.get_routes();
                let mut routes_sorted: Vec<(&String, &u32)> = routes.iter().collect();
                routes_sorted.sort_by(|a, b| b.1.cmp(a.1));
                routes_sorted.truncate(10);
                for route in routes_sorted {
                    println!("La ruta {:?} fue solicitada {:?} veces", route.0, route.1);
                }
                println!("----------------------------------------------------------");
                drop(stats_block);
            }
            Err(e) => {
                println!("Algo salió mal con el stats_lock. Error: {}", e);
            }
        }

        thread::sleep(Duration::from_millis(STATS_LOG_PERIOD * 1000));
        if let Ok(stats_signal) = processing_reserves_rx.try_recv() {
            processing = stats_signal;
        }
    }
}

/**
 * Parse the reserves stored in a specific file.
 * Recieves file system path
 */
pub fn parse_reserves(
    processing_reserves_tx: Sender<bool>,
    filename: &str,
    stats_mutex: Arc<Mutex<Stats>>,
) {
    let mut reserves = vec![];
    let mut airlines = AirlinesSemaphore::new();
    airlines.insert_airline_semaphore("Aerolineas_Argentinas".to_string());
    airlines.insert_airline_semaphore("LAN".to_string());
    let airlines_semaphore = Arc::new(airlines);
    let hotel_sem = Arc::new(Semaphore::new(WEBSERVICE_HOTEL_LIMIT));
    if let Ok(lines) = read_lines(filename) {
        // Consumes the iterator, returns an (Optional) String
        for reserve_line in lines.into_iter().flatten() {
            let reserve_split: Vec<&str> = reserve_line.split(' ').collect();
            let origin = reserve_split[0].to_string();
            let destination = reserve_split[1].to_string();
            let airline = reserve_split[2].to_string();
            let hotel = reserve_split[3].to_string();
            let airline_semaphore_for_process = airlines_semaphore.clone();
            let route = Route::new(origin.clone(), destination.clone());
            let stat_mutex_for_stats = stats_mutex.clone();
            let stat_mutex_for_fligth = stats_mutex.clone();
            let stat_mutex_for_package = stats_mutex.clone();
            if hotel == NO_HOTEL {
                reserves.push(thread::spawn(move || {
                    process_flight(
                        &Flight::new(origin, destination, airline),
                        airline_semaphore_for_process,
                        stat_mutex_for_fligth,
                    )
                }));
            } else {
                let hotel_sem_clone = hotel_sem.clone();
                reserves.push(thread::spawn(move || {
                    process_package(
                        &Package::new(origin, destination, airline, hotel),
                        airline_semaphore_for_process,
                        hotel_sem_clone,
                        stat_mutex_for_package,
                    )
                }));
            }
            reserves.push(thread::spawn(move || {
                increment_stats(stat_mutex_for_stats, route)
            }));
        }
        for child in reserves {
            let _ = child.join();
        }
        if let Err(error) = processing_reserves_tx.send(false){
            println!("Ocurrio un error enviando a traves del channel: {}", error);
        }
    }
}

/**
 * The output is wrapped in a Result to allow matching on errors
 * Returns an Iterator to the Reader of the lines of the file.
 */
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

/**
 * Increment route counter for Stats
 */
pub fn increment_stats(stat_mutex: Arc<Mutex<Stats>>, route: Route) {
    let stats_block_result = stat_mutex.lock();
    match stats_block_result {
        Ok(mut stats_block) => {stats_block.increment_route_counter(route);}
        Err(e) => {println!("Algo salió mal con el stats_lock. Error: {}", e);}
    }
}

pub fn process_flight(
    flight: &Flight,
    airlines_semaphore: Arc<AirlinesSemaphore>,
    stat_mutex: Arc<Mutex<Stats>>,
) {
    let initial_process_time = SystemTime::now();
    let origin = flight.get_origin();
    let destination = flight.get_destination();
    let airline = flight.get_airline();
    logger::log(format!(
        "Procesando reserva de vuelo con origen {}, destino {} para la aerolinea {}",
        origin, destination, airline
    ));
    let _ = thread::spawn(move || {
        reserve_airline(&origin, &destination, &airline, &airlines_semaphore)
    })
    .join();
    let final_process_time = SystemTime::now();
    let difference = final_process_time
        .duration_since(initial_process_time)
        .expect("Ocurrio un error inesperado");
    println!(
        "La reserva de vuelo se proceso en {:?} segundo(s)",
        difference.as_secs()
    );
    let stats_block_result = stat_mutex.lock();
    match stats_block_result {
        Ok(mut stats_block) => {stats_block.add_reserve_processing_time(difference.as_secs());}
        Err(e) => {println!("Algo salió mal con el stats_lock. Error: {}", e);}
    }
}

pub fn process_package(
    package: &Package,
    airlines_semaphore: Arc<AirlinesSemaphore>,
    hotel_sem: Arc<Semaphore>,
    stat_mutex: Arc<Mutex<Stats>>,
) {
    let initial_process_time = SystemTime::now();
    let mut children = vec![];
    let origin = package.get_origin();
    let destination = package.get_destination();
    let airline = package.get_airline();
    let hotel = package.get_hotel();
    logger::log(format!(
        "Procesando reserva de paquete con origen {}, destino {} para la aerolinea {} y hotel {}",
        origin, destination, airline, hotel
    ));
    children.push(thread::spawn(move || {
        reserve_airline(&origin, &destination, &airline, &airlines_semaphore)
    }));
    children.push(thread::spawn(move || reserve_hotel(&hotel, &hotel_sem)));
    for child in children {
        let _ = child.join();
    }
    let final_process_time = SystemTime::now();
    let difference = final_process_time
        .duration_since(initial_process_time)
        .expect("Ocurrio un error inesperado");
    println!(
        "La reserva de paquete se proceso en {:?} segundo(s)",
        difference.as_secs()
    );
    let stats_block_result = stat_mutex.lock();
    match stats_block_result {
        Ok(mut stats_block) => {stats_block.add_reserve_processing_time(difference.as_secs());}
        Err(e) => {println!("Algo salió mal con el stats_lock. Error: {}", e);}
    }
}

pub fn reserve_airline(
    origin: &str,
    destination: &str,
    airline: &str,
    airlines_semaphore: &Arc<AirlinesSemaphore>,
) {
    match airlines_semaphore.get_airline_semaphore(airline.to_string()) {
        Some(airline_sem) => {
            airline_sem.acquire();
            logger::log(format!(
                "Solcitando reserva con origen {} y destino {} a la aerolinea {}",
                origin, destination, airline
            ));
            let approved: bool =
                webservice_aerolineas::reservar(origin.to_string(), destination.to_string());
            airline_sem.release();
            if !approved {
                logger::log(format!("La aerolinea {} no aprobó la reserva con origen {} y destino {}. Reintentando en {} segundos", airline, origin, destination, DELAY_BETWEEN_RETRIES_SECONDS));
                thread::sleep(Duration::from_millis(DELAY_BETWEEN_RETRIES_SECONDS * 1000));
                reserve_airline(origin, destination, airline, airlines_semaphore);
                return;
            }
            logger::log(format!(
                "La aerolinea {} aprobó la reserva con origen: {} y destino: {}",
                airline, origin, destination
            ));
        }
        _ => logger::log(format!(
            "No se pudo procesar la reserva con origen {} y destino {} para la aerolinea: {}",
            origin, destination, airline
        )),
    };
}

pub fn reserve_hotel(hotel: &str, hotel_sem: &Arc<Semaphore>) {
    hotel_sem.access();
    logger::log(format!("Solcitando reserva al hotel {}", hotel));
    webservice_hoteles::reservar(hotel.to_string());
    logger::log(format!(
        "El servicio de hoteles aprobó la reserva en: {}",
        hotel
    ));
}
