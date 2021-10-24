mod model;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use actix::prelude::*;
use model::receiver_actor::ReserveString;
use crate::model::ping_actor::PingActor;
use crate::model::ping_actor::Ping;
use crate::model::receiver_actor::ReceiverActor;
use crate::model::reserve_actor::ReserveActor;
use crate::model::logger;


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
async fn reserve(req_body: String) -> impl Responder {
    logger::log(format!("Recibiendo solicitud par procesar la reserva {}", req_body.clone()));
    let addr = ReceiverActor::new(ReserveActor.start()).start();
    let result = addr.send(ReserveString::new(req_body.clone())).await;
    match result {
        Ok(_res) => logger::log(format!("Reserva {} procesada con exito", req_body.clone())),
        Err(err) => logger::log(format!("Ocurrio un error al procesar la reserva {}. {}", req_body.clone(), err)),
    } 
    HttpResponse::Ok().body(req_body)
}

async fn version() -> impl Responder {
    HttpResponse::Ok().body("Version 1.0.0")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(ping)
            .service(reserve)
            .route("/version", web::get().to(version))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
