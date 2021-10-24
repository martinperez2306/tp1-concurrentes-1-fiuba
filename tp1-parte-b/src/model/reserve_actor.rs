use actix::prelude::*;
use crate::model::reserve::Reserve;
use crate::model::airline_ws_actor::{ReserveFlight};
use crate::model::hotel_ws_actor::{HotelWsActor, ReserveHotel};
use crate::model::airline_arbiters::AirlinesArbiters;
use actix::{Actor, Handler, Message, SyncArbiter, System, SyncContext};

const NO_HOTEL: &str = "-";

// Define actor
pub struct ReserveActor;

// Provide Actor implementation for our actor
impl Actor for ReserveActor {
    type Context = Context<Self>;

    fn started(&mut self, _ctx: &mut Context<Self>) {
       println!("ReserveActor is alive");
    }

    fn stopped(&mut self, _ctx: &mut Context<Self>) {
       println!("ReserveActor is stopped");
    }
}

/// Define handler for `Reserve` message
impl Handler<Reserve> for ReserveActor {
    type Result = Result<bool, std::io::Error>;

    fn handle(&mut self, msg: Reserve, _ctx: &mut Context<Self>) -> Self::Result {
        process_reserve(msg);
        Ok(true)
    }
}

async fn process_flight(airlines: AirlinesArbiters, airline: String, origin: String, destination: String) {
    match airlines.get_airline_arbiter(airline.to_string()) {
        Some(airline_arbiter) => {
            airline_arbiter.send(ReserveFlight(origin, destination)).await.unwrap();
        }
        _ => println!("No se encontró la aerolinea: {}", airline),
    }
}

async fn process_reserve(reserve: Reserve) {
    let origin = reserve.get_origin();
    let destination = reserve.get_destination();
    let airline = reserve.get_airline();
    let hotel = reserve.get_hotel();
    let arbitrer_hotel = SyncArbiter::start(1, || HotelWsActor { id: "KEP".to_string() });
    let mut airlines = AirlinesArbiters::new();
    airlines.insert_airline_arbiter("Aerolineas_Argentinas".to_string());
    airlines.insert_airline_arbiter("LAN".to_string());
    if hotel == NO_HOTEL {
        println!("Procesar Vuelo con Origen {}, Destino {} y Aerolinea {}", origin, destination, airline);
        // Search for airline
        process_flight(airlines, airline, origin, destination).await;
    } else {
        println!("Procesar Paquete con Origen {}, Destino {}, Aerolinea {} y Hotel {}", origin, destination, airline, hotel);
        // Search for airline
        process_flight(airlines, airline, origin, destination).await;
        // Hotel ws call
        arbitrer_hotel.send(ReserveHotel(hotel)).await.unwrap();
    }
}