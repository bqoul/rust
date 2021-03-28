use serde_derive::Deserialize;

#[derive(Deserialize)]
pub struct Forecast {
    pub location: Location,
    pub current: Current,
}

#[derive(Deserialize)]
pub struct Location {
    pub name: String,
    pub country: String,
}

#[derive(Deserialize)]
pub struct Current {
    pub temp_c: f64,
    pub temp_f: f64,
    pub condition: Condition,
}

#[derive(Deserialize)]
pub struct Condition {
    pub text: String,
}

#[derive(Deserialize)]
pub struct Failed {
    pub error: Error,
}

#[derive(Deserialize)]
pub struct Error {
    pub message: String,
}
