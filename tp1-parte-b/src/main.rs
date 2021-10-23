mod model;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use actix::prelude::*;
use crate::model::receiver_actor::ReceiverActor;
use crate::model::receiver_actor::Ping;

#[get("/ping")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Pong!")
}

#[post("/reserves")]
async fn echo(req_body: String) -> impl Responder {
    let addr = ReceiverActor.start();
    // Send Ping message.
    // send() message returns Future object, that resolves to message result
    let result = addr.send(Ping).await;

    match result {
        Ok(res) => println!("Got result: {}", res.unwrap()),
        Err(err) => println!("Got error: {}", err),
    } 
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
