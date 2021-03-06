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
    ALL,
    MP,
    V,
    L,
    SD,
    S,
    KD,
    M,
    C,
    Other
}

impl From<Affiliation> for String {
    fn from(a: Affiliation) -> Self {
        match a {
            Affiliation::ALL => "ALL".to_owned(),
            Affiliation::MP => "MP".to_owned(),
            Affiliation::V => "V".to_owned(),
            Affiliation::L => "L".to_owned(),
            Affiliation::SD => "SD".to_owned(),
            Affiliation::S => "S".to_owned(),
            Affiliation::KD => "KD".to_owned(),
            Affiliation::M => "M".to_owned(),
            Affiliation::C => "C".to_owned(),
            Affiliation::Other => "Other".to_owned()
        }
    }
}

impl Into<Affiliation> for String {
    fn into(self) -> Affiliation {
        match self.as_str() {
            "MP" => Affiliation::MP,
            "V" => Affiliation::V,
            "SD" => Affiliation::SD,
            "S" => Affiliation::S,
            "KD" => Affiliation::KD,
            "M" => Affiliation::M,
            "C" => Affiliation::C,
            "L" => Affiliation::L,
            "ALL" => Affiliation::ALL,
            _ => Affiliation::Other

        }

    }
}

#[derive(Deserialize)]
pub struct AnforandeRequest {
    pub affiliation: Affiliation,
}

#[derive(Debug)]
pub struct TextTime {
    pub time: chrono::NaiveDateTime,
    pub content: String
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PrefixSum {
    pub counts: Vec<HashMap<String, f64>>,
    pub dates: Vec<NaiveDateTime>
}
