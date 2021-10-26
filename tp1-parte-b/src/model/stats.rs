use crate::model::route::Route;
use std::{collections::HashMap, time::Duration};
use actix::prelude::*;

#[derive(Message)]
#[rtype(result = "Result<(), std::io::Error>")]
pub struct GetStats;

#[derive(Message)]
#[rtype(result = "Result<(), std::io::Error>")]
pub struct UpdateStats {
    pub route: Route,
    pub process_time: Duration
}

/// This struct keeps track of statics about the most required routes and the average processing time
/// of the reserves.
pub struct Stats {
    routes: HashMap<String, u32>,
    reserve_processing_times: Vec<u64>,
}

impl Stats {
    pub fn new() -> Stats {
        let routes = HashMap::new();
        let reserve_processing_times = vec![];
        Stats {
            routes,
            reserve_processing_times,
        }
    }
    pub fn increment_route_counter(&mut self, route: Route) {
        let route_id = route.get_id();
        let count = self.routes.entry(route_id).or_insert(0);
        *count += 1;
    }
    pub fn get_routes(&self) -> HashMap<String, u32> {
        self.routes.clone()
    }
    fn get_reserve_processing_time(&self) -> Vec<u64> {
        self.reserve_processing_times.clone()
    }
    pub fn add_reserve_processing_time(&mut self, time: u64) {
        self.reserve_processing_times.push(time);
    }
    pub fn get_avg_reserve_processing_time(&self) -> u64 {
        let mut avg: u64 = 0;
        let mut count: u64 = 0;
        for time in self.get_reserve_processing_time() {
            count += 1;
            avg += time;
        }
        if count != 0{
            avg /= count;
        }
        avg
    }
}

// Provide Actor implementation for our actor
impl Actor for Stats {
    type Context = SyncContext<Self>;

    fn started(&mut self, _ctx: &mut SyncContext<Self>) {
       println!("Stats Actor is alive");
    }

    fn stopped(&mut self, _ctx: &mut SyncContext<Self>) {
       println!("Stats Actor is stopped");
    }
}

/// Define handler for `Reserve String` message
impl Handler<GetStats> for Stats {
    type Result = Result<(), std::io::Error>;

    fn handle(&mut self, _msg: GetStats, _ctx: &mut <Stats as Actor>::Context) -> Self::Result {
        println!("---------------LAS 10 RUTAS MAS SOLICITADAS---------------");
        let routes: HashMap<String, u32> = self.get_routes();
        let mut routes_sorted: Vec<(&String, &u32)> = routes.iter().collect();
        routes_sorted.sort_by(|a, b| b.1.cmp(a.1));
        routes_sorted.truncate(10);
        for route in routes_sorted {
            println!("La ruta {:?} fue solicitada {:?} veces", route.0, route.1);
        }
        println!("----------------------------------------------------------");
        println!("-----------------------TIEMPO MEDIO-----------------------");
        let avg_reserve_processing_time = self.get_avg_reserve_processing_time();
        println!(
            "El tiempo medio de procesamiento de una reserva es {} segundos",
            avg_reserve_processing_time
        );
        println!("----------------------------------------------------------");
        Ok(())
    }
}

/// Define handler for `Reserve String` message
impl Handler<UpdateStats> for Stats {
    type Result = Result<(), std::io::Error>;

    fn handle(&mut self, msg: UpdateStats, _ctx: &mut <Stats as Actor>::Context) -> Self::Result {
        self.increment_route_counter(msg.route);
        self.add_reserve_processing_time(msg.process_time.as_secs());
        Ok(())
    }
}
impl Default for Stats {
    fn default() -> Self {
        Self::new()
    }
}
