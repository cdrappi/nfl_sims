use serde::Deserialize;

#[derive(Debug, Clone, PartialEq, Hash, Deserialize)]
pub enum StadiumType {
    Dome,
    Open,
    Outdoor,
}

#[derive(Clone, Debug)]
pub struct Weather {
    pub stadium_type: StadiumType,
    pub start_time: String,
}
