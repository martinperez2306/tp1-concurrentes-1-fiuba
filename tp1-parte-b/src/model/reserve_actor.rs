use actix::prelude::*;
use crate::model::logger;
use crate::model::reserve::Reserve;
use crate::model::airline_ws_actor::{ReserveFlight};
use crate::model::hotel_ws_actor::{HotelWsActor, ReserveHotel};
use crate::model::airline_arbiters::AirlinesArbiters;
use crate::model::route::Route;
use actix::{Actor, Handler};

use super::flight::{Flight};
use super::package::Package;
use super::stats::{Stats, UpdateStats};

const NO_HOTEL: &str = "-";

/// Define message
#[derive(Message)]
#[rtype(result = "Result<bool, std::io::Error>")]
pub struct ReserveMsg{
    reserve: Reserve,
    arbiter_hotel: Addr<HotelWsActor>,
    arbiter_airlines: AirlinesArbiters,
    arbiter_stats: Addr<Stats>
}

impl ReserveMsg {
    pub fn new(reserve: Reserve,
               arbiter_hotel: Addr<HotelWsActor>,
               arbiter_airlines: AirlinesArbiters,
               arbiter_stats: Addr<Stats> ) -> ReserveMsg{
        ReserveMsg {
            reserve,
            arbiter_hotel,
            arbiter_airlines,
            arbiter_stats
        }
    }
}

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

/// Define handler for `ReserveMsg` message
impl Handler<ReserveMsg> for ReserveActor {
    type Result = ResponseFuture<Result<bool, std::io::Error>>;

    fn handle(&mut self, msg: ReserveMsg, _ctx: &mut Context<Self>) -> Self::Result {
        Box::pin(async move {
            let _result = process_reserve(msg.reserve, msg.arbiter_hotel, msg.arbiter_airlines, msg.arbiter_stats).await;
            Ok(true)
        })
    }
}

async fn process_flight(airlines: AirlinesArbiters, flight: Flight) {
    let airline = flight.get_airline().to_string();
    match airlines.get_airline_arbiter(airline.to_string()) {
        Some(airline_arbiter) => {
            // Search for airline
            airline_arbiter.send(ReserveFlight(flight.get_route().get_origin(), flight.get_route().get_destination())).await.unwrap();
        }
        _ => println!("No se encontró la aerolinea: {}", airline),
    }
}

async fn process_package(airlines: AirlinesArbiters, arbitrer_hotel: Addr<HotelWsActor>, package: Package) {
    let flight = Flight::new(Route::new(package.get_route().get_origin(), package.get_route().get_destination()),package.get_airline());
    // Search for airline
    let process_airline = process_flight(airlines, flight);
    // Hotel ws call
    let process_hotel = arbitrer_hotel.send(ReserveHotel(package.get_hotel()));
    process_airline.await;
    process_hotel.await.unwrap();
}

async fn process_reserve(reserve: Reserve, arbiter_hotel: Addr<HotelWsActor>, arbiter_airlines: AirlinesArbiters, arbiter_stats: Addr<Stats>) {
    let origin = reserve.get_origin();
    let destination = reserve.get_destination();
    let airline = reserve.get_airline();
    let hotel = reserve.get_hotel();
    if hotel == NO_HOTEL {
        logger::log(format!("Procesando Vuelo con Origen {}, Destino {} y Aerolinea {}", origin, destination, airline));
        process_flight(arbiter_airlines, Flight::new(Route::new(origin.clone(), destination.clone()), airline)).await;
    } else {
        logger::log(format!("Procesando Paquete con Origen {}, Destino {}, Aerolinea {} y Hotel {}", origin, destination, airline, hotel));
        process_package(arbiter_airlines, arbiter_hotel, Package::new(Route::new(origin.clone(), destination.clone()), airline, hotel)).await;
    }
    let _ = arbiter_stats.send(UpdateStats{route: Route::new(origin, destination) }).await;
}