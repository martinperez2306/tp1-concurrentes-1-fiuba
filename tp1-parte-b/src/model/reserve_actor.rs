use std::time::SystemTime;

use crate::model::airline_arbiters::AirlinesArbiters;
use crate::model::airline_ws_actor::ReserveFlight;
use crate::model::hotel_ws_actor::{HotelWsActor, ReserveHotel};
use crate::model::logger;
use crate::model::reserve::Reserve;
use crate::model::route::Route;
use actix::prelude::*;
use actix::{Actor, Handler};

use super::flight::{Flight};
use super::package::Package;
use super::stats::{Stats, UpdateStats};

const NO_HOTEL: &str = "-";

#[derive(Message)]
#[rtype(result = "Result<bool, std::io::Error>")]
pub struct ReserveMsg {
    reserve: Reserve,
    arbiter_hotel: Addr<HotelWsActor>,
    arbiter_airlines: AirlinesArbiters,
    arbiter_stats: Addr<Stats>,
}

impl ReserveMsg {
    pub fn new(
        reserve: Reserve,
        arbiter_hotel: Addr<HotelWsActor>,
        arbiter_airlines: AirlinesArbiters,
        arbiter_stats: Addr<Stats>,
    ) -> ReserveMsg {
        ReserveMsg {
            reserve,
            arbiter_hotel,
            arbiter_airlines,
            arbiter_stats,
        }
    }
}

#[derive(Message)]
#[rtype(result = "Result<bool, std::io::Error>")]
pub struct ProcessReserve {
    reserve: Reserve,
    arbiter_hotel: Addr<HotelWsActor>,
    arbiter_airlines: AirlinesArbiters,
    arbiter_stats: Addr<Stats>,
}

impl ProcessReserve {
    pub fn new(
        reserve: Reserve,
        arbiter_hotel: Addr<HotelWsActor>,
        arbiter_airlines: AirlinesArbiters,
        arbiter_stats: Addr<Stats>,
    ) -> ProcessReserve {
        ProcessReserve {
            reserve,
            arbiter_hotel,
            arbiter_airlines,
            arbiter_stats,
        }
    }
}

#[derive(Message)]
#[rtype(result = "Result<bool, std::io::Error>")]
pub struct ProcessPackage {
    airlines: AirlinesArbiters,
    arbitrer_hotel: Addr<HotelWsActor>,
    package: Package,
}

impl ProcessPackage {
    pub fn new(
        airlines: AirlinesArbiters,
        arbitrer_hotel: Addr<HotelWsActor>,
        package: Package
    ) -> ProcessPackage {
        ProcessPackage {
            airlines,
            arbitrer_hotel,
            package
        }
    }
}

#[derive(Message)]
#[rtype(result = "Result<bool, std::io::Error>")]
pub struct ProcessFlight {
    arbiter_airlines: AirlinesArbiters,
    flight: Flight
}

impl ProcessFlight {
    pub fn new(
        arbiter_airlines: AirlinesArbiters,
        flight: Flight
    ) -> ProcessFlight {
        ProcessFlight {
            arbiter_airlines,
            flight,
        }
    }
}

/// This Actor is responsible for processing the information present in the received Reserve accordingly
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

    fn handle(&mut self, msg: ReserveMsg, ctx: &mut Context<Self>) -> Self::Result {
        let addr = ctx.address();
        Box::pin(async move {
            let _result = addr.send(ProcessReserve::new(msg.reserve,
                msg.arbiter_hotel,
                msg.arbiter_airlines,
                msg.arbiter_stats,))
            .await;
            Ok(true)
        })
    }
}

/// Define handler for `ProcessReserve` message
impl Handler<ProcessReserve> for ReserveActor {
    type Result = ResponseFuture<Result<bool, std::io::Error>>;

    fn handle(&mut self, msg: ProcessReserve, ctx: &mut Context<Self>) -> Self::Result {
        let addr = ctx.address();
        let reserve = msg.reserve;
        let arbiter_airlines = msg.arbiter_airlines;
        let arbiter_hotel = msg.arbiter_hotel;
        let arbiter_stats = msg.arbiter_stats;
        Box::pin(async move {
            let initial_process_time = SystemTime::now();
            let origin = reserve.get_origin();
            let destination = reserve.get_destination();
            let airline = reserve.get_airline();
            let hotel = reserve.get_hotel();
            if hotel == NO_HOTEL {
                logger::log(format!(
                    "Procesando Vuelo con Origen {}, Destino {} y Aerolinea {}",
                    origin, destination, airline
                ));
                let flight = Flight::new(Route::new(origin.clone(), destination.clone()), airline);
                let _result_process_flight = addr.send(ProcessFlight::new(arbiter_airlines, flight)).await;
            } else {
                logger::log(format!(
                    "Procesando Paquete con Origen {}, Destino {}, Aerolinea {} y Hotel {}",
                    origin, destination, airline, hotel
                ));
                let _result_process_package = addr.send(ProcessPackage::new(arbiter_airlines,
                    arbiter_hotel,
                    Package::new(
                    Route::new(origin.clone(), destination.clone()),
                        airline,
                        hotel,
                    )))
                .await;
            }
            let final_process_time = SystemTime::now();
            let difference = final_process_time
                .duration_since(initial_process_time)
                .expect("Ocurrio un error inesperado");
            let _ = arbiter_stats
                .try_send(UpdateStats {
                    route: Route::new(origin, destination),
                    process_time: difference,
                });
            Ok(true)
        })
    }
}

/// Define handler for `ProcessPackage` message
impl Handler<ProcessPackage> for ReserveActor {
    type Result = ResponseFuture<Result<bool, std::io::Error>>;

    fn handle(&mut self, msg: ProcessPackage, ctx: &mut Context<Self>) -> Self::Result {
        let addr = ctx.address();
        let airlines = msg.airlines;
        let flight = Flight::new(msg.package.get_route(), msg.package.get_airline());
        let arbitrer_hotel = msg.arbitrer_hotel;
        let hotel = msg.package.get_hotel();
        Box::pin(async move {
            let process_airline = addr.send(ProcessFlight::new(airlines, flight));
            let process_hotel = arbitrer_hotel.send(ReserveHotel(hotel));
            let _result_airline = process_airline.await;
            let _result_hotel= process_hotel.await;
            Ok(true)
        })
    }
}

/// Define handler for `ProcessFlight` message
impl Handler<ProcessFlight> for ReserveActor {
    type Result = ResponseFuture<Result<bool, std::io::Error>>;

    fn handle(&mut self, msg: ProcessFlight, ctx: &mut Context<Self>) -> Self::Result {
        let addr = ctx.address();
        let airlines = msg.arbiter_airlines;
        let flight = msg.flight;
        Box::pin(async move {
            let airline = flight.get_airline().to_string();
            match airlines.get_airline_arbiter(airline.to_string()) {
                Some(airline_arbiter) => {
                    // Search for airline
                    let result = airline_arbiter
                        .send(ReserveFlight(
                            flight.get_route().get_origin(),
                            flight.get_route().get_destination(),
                        ))
                        .await;
                    let approve = result.unwrap();
                    if !approve {
                        let _result_process_flight = addr.send(ProcessFlight::new(airlines, flight)).await;
                    }
                }
                _ => println!("No se encontró la aerolinea: {}", airline),
            }
            Ok(true)
        })
    }
}
