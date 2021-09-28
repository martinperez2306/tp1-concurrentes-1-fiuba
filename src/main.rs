mod webservice_aerolineas;
mod webservice_hoteles;
use std::{thread, time::Duration};

const DELAY_BETWEEN_RETRIES: u64 = 5;

fn llamar_ws_aerolineas() {
    let aprobado: bool = webservice_aerolineas::reservar("bsas".to_string(), "san pablo".to_string());
    if !aprobado {
        println!("La aerolinea no aprobó la reserva. Reintentando en {} segundos", DELAY_BETWEEN_RETRIES);
        thread::sleep(Duration::from_millis(DELAY_BETWEEN_RETRIES*1000));
        println!("Reintentando...");
        llamar_ws_aerolineas();
        return;
    }
    println!("La aerolinea aprobó la reserva!");
}

fn llamar_ws_hoteles() {
    webservice_hoteles::reservar("san pablo".to_string());
    println!("El hotel aprobó la reserva!");
}

fn main() {
    println!("Solicitando reserva al ws de aerolineas");
    llamar_ws_aerolineas();
    println!("Solicitando reserva al ws de hoteles");
    llamar_ws_hoteles();
}