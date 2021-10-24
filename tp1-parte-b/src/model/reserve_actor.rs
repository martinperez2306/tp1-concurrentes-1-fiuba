use actix::prelude::*;
use crate::model::reserve::Reserve;
use crate::model::airline_ws_actor::{ReserveFlight};
use crate::model::hotel_ws_actor::{HotelWsActor, ReserveHotel};
use crate::model::airline_arbiters::AirlinesArbiters;
use crate::model::route::Route;
use actix::{Actor, Handler, SyncArbiter};

use super::package::Package;

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
    type Result = ResponseFuture<Result<bool, std::io::Error>>;

    fn handle(&mut self, msg: Reserve, _ctx: &mut Context<Self>) -> Self::Result {
        Box::pin(async move {
            let _result = process_reserve(msg).await;
            Ok(true)
        })
    }
}

async fn process_flight(airlines: AirlinesArbiters, airline: String, origin: String, destination: String) {
    match airlines.get_airline_arbiter(airline.to_string()) {
        Some(airline_arbiter) => {
            // Search for airline
            airline_arbiter.send(ReserveFlight(origin, destination)).await.unwrap();
        }
        _ => println!("No se encontró la aerolinea: {}", airline),
    }
}

async fn process_package(airlines: AirlinesArbiters, arbitrer_hotel: Addr<HotelWsActor>, package: Package) {
    // Search for airline
    let process_airline = process_flight(airlines, package.get_airline(), package.get_route().get_origin(), package.get_route().get_destination());
    // Hotel ws call
    let process_hotel = arbitrer_hotel.send(ReserveHotel(package.get_hotel()));
    process_airline.await;
    process_hotel.await.unwrap();
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
        process_flight(airlines, airline, origin, destination).await;
    } else {
        println!("Procesar Paquete con Origen {}, Destino {}, Aerolinea {} y Hotel {}", origin, destination, airline, hotel);
        process_package(airlines, arbitrer_hotel, Package::new(Route::new(origin, destination), airline, hotel)).await;
    }
}