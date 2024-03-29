use std::collections::HashMap;
use std::sync::Arc;
use std_semaphore::Semaphore;
/// How many seconds to wait until trying again after a airline web service reservation fails
const WEBSERVICE_AIRLINE_LIMIT: isize = 10;

/// This struct maps a given airline with its corresponding Semaphore.
pub struct AirlinesSemaphore {
    semaphores_by_airline: HashMap<String, Arc<Semaphore>>,
}

impl AirlinesSemaphore {
    pub fn new() -> AirlinesSemaphore {
        let semaphores_by_airline = HashMap::new();
        AirlinesSemaphore {
            semaphores_by_airline,
        }
    }
    pub fn get_airline_semaphore(&self, airline: String) -> Option<&Arc<Semaphore>> {
        self.semaphores_by_airline.get(&airline)
    }
    pub fn insert_airline_semaphore(&mut self, airline: String) {
        self.semaphores_by_airline
            .insert(airline, Arc::new(Semaphore::new(WEBSERVICE_AIRLINE_LIMIT)));
    }
}
