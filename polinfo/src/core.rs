use chrono::prelude::NaiveDateTime;
use config;
use config::ConfigError;
use serde::Deserialize;
use serde::Serialize;
use derive_more::From;
use std::collections::HashMap;

#[derive(Deserialize)]
pub struct Config {
    pub server_addr: String,
    pub pg: deadpool_postgres::Config,
}

impl Config {
    pub fn from_env() -> Result<Self, ConfigError> {
        let mut cfg = config::Config::new();
        cfg.merge(config::Environment::new())?;
        cfg.try_into()
    }
}

#[derive(Deserialize, Clone, From)]
pub enum Affiliation {
    MP,
    V,
    L,
    SD,
    S,
    KD,
    M,
    C,
}

impl From<Affiliation> for String {
    fn from(a: Affiliation) -> Self {
        match a {
            Affiliation::MP => "MP".to_owned(),
            Affiliation::V => "V".to_owned(),
            Affiliation::L => "L".to_owned(),
            Affiliation::SD => "SD".to_owned(),
            Affiliation::S => "S".to_owned(),
            Affiliation::KD => "KD".to_owned(),
            Affiliation::M => "M".to_owned(),
            Affiliation::C => "C".to_owned(),
        }
    }
}

#[derive(Deserialize)]
pub struct AnforandeRequest {
    pub affiliation: Affiliation,
}

#[derive(Debug)]
pub struct Anforande {
    pub time: chrono::NaiveDateTime,
    pub content: String
}

#[derive(Serialize, Debug, Clone)]
pub struct PrefixSum {
    pub counts: Vec<HashMap<String, u64>>,
    pub dates: Vec<NaiveDateTime>
}
