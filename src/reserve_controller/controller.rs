pub trait Controller {
    fn reserve_flight(&'static self, origin: String, destination: String, airline: String);
    fn reserve_package(&'static self, origin: String, destination: String, airline: String, hotel: String);
}
