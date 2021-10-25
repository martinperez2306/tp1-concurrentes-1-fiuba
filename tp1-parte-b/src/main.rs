mod model;
use std::thread;
use std::time::Duration;

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use actix::prelude::*;
use model::receiver_actor::ReserveString;
use model::stats::Stats;
use crate::model::ping_actor::PingActor;
use crate::model::ping_actor::Ping;
use crate::model::receiver_actor::ReceiverActor;
use crate::model::reserve_actor::ReserveActor;
use crate::model::logger;
use crate::model::airline_arbiters::AirlinesArbiters;
use crate::model::hotel_ws_actor::HotelWsActor;
use crate::model::stats::GetStats;


#[get("/ping")]
async fn ping() -> impl Responder {
    let addr = PingActor.start();
    // Send Ping message.
    // send() message returns Future object, that resolves to message result
    let result = addr.send(Ping).await;

    match result {
        Ok(res) => println!("Got result: {}", res.unwrap()),
        Err(err) => println!("Got error: {}", err),
    } 

    HttpResponse::Ok().body("Pong!")
}

#[post("/reserves")]
async fn reserve(req_body: String, data: web::Data<Arbiters>) -> impl Responder {
    logger::log(format!("Recibiendo solicitud par procesar la reserva {}", req_body.clone()));
    let addr = ReceiverActor::new(ReserveActor.start()).start();
    let result = addr.send(ReserveString::new(req_body.clone(), data.arbiter_hotel.clone(), data.arbiter_airlines.clone(), data.arbiter_stats.clone()))
                                                .await;
    match result {
        Ok(_res) => logger::log(format!("Reserva {} procesada con exito", req_body.clone())),
        Err(err) => logger::log(format!("Ocurrio un error al procesar la reserva {}. {}", req_body.clone(), err)),
    } 
    HttpResponse::Ok().body(req_body)
}

async fn version() -> impl Responder {
    HttpResponse::Ok().body("Version 1.0.0")
}

pub struct Arbiters {
    pub arbiter_hotel: Addr<HotelWsActor>,
    pub arbiter_airlines: AirlinesArbiters,
    pub arbiter_stats: Addr<Stats>
}

impl Arbiters {
    pub fn clone(&self) -> Arbiters {
        Arbiters {
            arbiter_hotel: self.arbiter_hotel.clone(),
            arbiter_airlines: self.arbiter_airlines.clone(),
            arbiter_stats: self.arbiter_stats.clone()
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let arbiter_hotel = SyncArbiter::start(1, || HotelWsActor { id: "KEP".to_string() });
    let arbiter_airlines = AirlinesArbiters::new();
    let arbiter_stats = SyncArbiter::start(1, || Stats::new());
    let arbiter_stats_clone = arbiter_stats.clone();
    let arbiters = web::Data::new(Arbiters {
        arbiter_hotel,
        arbiter_airlines,
        arbiter_stats,
    });
    thread::spawn(move || loop {
        let _result = arbiter_stats_clone.send(GetStats);
        thread::sleep(Duration::from_secs(5));
    });
    HttpServer::new( move || {
        App::new()
            .service(ping)
            .service(reserve)
            .app_data(arbiters.clone())
            .route("/version", web::get().to(version))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
