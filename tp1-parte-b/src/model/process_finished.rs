use actix::prelude::*;
/// Define message
#[derive(Message)]
#[rtype(result = "Result<bool, std::io::Error>")]
pub struct ProcessFinished {
    pub result: String
}