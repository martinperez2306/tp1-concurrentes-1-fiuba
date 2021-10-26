use std::collections::HashMap;
use crate::model::airline_ws_actor::AirlineWsActor;
use actix::{SyncArbiter, Addr};

const WEBSERVICE_AIRLINE_LIMIT: usize = 10;

/// It coordinates the AirlineWsActors so that the program never surpass the airline web service request limit
pub struct AirlinesArbiters {
    arbiters_by_airline: HashMap<String, Addr<AirlineWsActor>>,
}

impl AirlinesArbiters {
    pub fn new() -> AirlinesArbiters {
        let mut arbiters_by_airline = HashMap::new();
        arbiters_by_airline
            .insert("Aerolineas_Argentinas".to_string(), SyncArbiter::start(WEBSERVICE_AIRLINE_LIMIT, move || AirlineWsActor { id: "1".to_string() }));
        arbiters_by_airline
            .insert("LAN".to_string(), SyncArbiter::start(WEBSERVICE_AIRLINE_LIMIT, move || AirlineWsActor { id: "1".to_string() }));
        AirlinesArbiters {
            arbiters_by_airline,
        }
    }
    pub fn get_airline_arbiter(&self, airline: String) -> Option<&Addr<AirlineWsActor>> {
        self.arbiters_by_airline.get(&airline)
    }

    pub fn clone(&self) -> AirlinesArbiters{
        AirlinesArbiters {
            arbiters_by_airline: self.arbiters_by_airline.clone(),
        }
    }
}
impl Default for AirlinesArbiters {
    fn default() -> Self {
        Self::new()
    }
}
