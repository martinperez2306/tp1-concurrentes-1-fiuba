pub struct Route {
    origin: String,
    destination: String
}

impl Route{
    pub fn new(origin: String, destination: String) -> Route{
        Route{ origin, destination }
    }
    pub fn get_id(&self) -> String { 
        let route_id = self.origin.clone() + "_" + &self.destination.clone(); 
        return route_id 
    }
}