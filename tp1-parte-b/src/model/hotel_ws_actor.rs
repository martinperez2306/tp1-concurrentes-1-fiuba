extern crate actix;

use std::{thread};
use std::time::{Duration};
use rand::Rng;

use actix::{Actor, Handler, Message, SyncContext};


#[derive(Message)]
#[rtype(result = "bool")]
pub struct ReserveHotel(pub String);
pub struct HotelWsActor {
    pub id: String
}

impl Actor for HotelWsActor {
    type Context = SyncContext<Self>;
}

impl Handler<ReserveHotel> for HotelWsActor {
    type Result = bool;

    fn handle(&mut self, reserve: ReserveHotel, _ctx: &mut <HotelWsActor as Actor>::Context) -> Self::Result  {
        println!("Hotel: {} \n destino: {}", self.id, reserve.0);
        let mut rng = rand::thread_rng();
        let miliseconds_to_sleep = rng.gen_range(0..10);
        thread::sleep(Duration::from_millis(miliseconds_to_sleep * 1000));
        true
        /*
        Box::pin(sleep(Duration::from_secs(msg.0))
            .into_actor(self)
            .map(move |_result, me, _ctx| {
                println!("[{}] desperté de {}", me.id, msg.0);
            }))
        */
    }
}
