use config;
use config::ConfigError;
use serde::Deserialize;
use std::collections::{HashSet, HashMap};
use regex::Regex;

use std::path::PathBuf;
use std::fs::File;
use itertools::Itertools;

use std::io::BufRead;

#[derive(Deserialize)]
pub struct WordifierConfig {
    pub allowed_file: PathBuf,
    pub standard_file: PathBuf,
}

impl WordifierConfig {
    pub fn from_env() -> Result<Self, ConfigError> {
        let mut cfg = config::Config::new();
        cfg.merge(config::Environment::new())?;
        cfg.try_into()
    }
}

#[derive(Debug)]
pub enum WordifierError {
    ConfigError(ConfigError),
    IOError(std::io::Error)
}

impl From<std::io::Error> for WordifierError {
    fn from(err: std::io::Error) -> Self {
        WordifierError::IOError(err)
    }
}

impl From<ConfigError> for WordifierError {
    fn from(err: ConfigError) -> Self {
        WordifierError::ConfigError(err)
    }
}

pub struct Wordifier {
    // Words which we calculate Prefix Sums for
    allowed: HashSet<String>,

    // Standardization mapping of strings
    // E.g 
    // Cookies -> Cookie
    // Cookie -> Cookie
    standard: HashMap<String, String>
}

impl Wordifier {
    fn parse_allowed(reader: std::io::BufReader<File>) 
        -> Result<HashSet<String>, WordifierError> {
            let allowed: HashSet<String> = reader
                .lines()
                .map(|word| word.unwrap().to_lowercase())
                .collect();

            Ok(allowed)

    }

    fn parse_standard(reader: std::io::BufReader<File>)
        -> Result<HashMap<String, String>, WordifierError> {
            let standard: HashMap<String, String> = reader
                .lines()
                .map(|line| line
                    .unwrap()
                    .split_whitespace()
                    .map(|word| word.to_owned())
                    .next_tuple()
                    .unwrap())
                .collect();
            Ok(standard)
    }

    pub fn new() -> Result<Self, WordifierError> {
        let config = WordifierConfig::from_env()?;

        let allowed_file = File::open(config.allowed_file)?;
        let standard_file = File::open(config.standard_file)?;

        let allowed = Wordifier::parse_allowed(
            std::io::BufReader::new(allowed_file))?;
        let standard = Wordifier::parse_standard(
            std::io::BufReader::new(standard_file))?;


        Ok(
            Wordifier{
                allowed,
                standard,
            })
    }

    pub fn words(&self, s: &String) -> Vec<String> {
        let re = Regex::new("\\.,!\\?-':;").unwrap();
        
        re.replace_all(
            s.to_lowercase().as_str(), " ")
            .to_string()
            .split_whitespace()
            .map(|word| word.trim().to_owned())
            .map(|w| self.standard.get(&w).unwrap_or(&w).to_owned())
            .filter(|w| self.allowed.contains(w))
            .collect()

    }
}

