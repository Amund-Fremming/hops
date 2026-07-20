use once_cell::sync::Lazy;
use regex::Regex;

pub mod adapters;
pub mod config;
pub mod db;
pub mod error;
pub mod handlers;
pub mod models;
pub mod ports;
pub mod services;
pub mod state;

pub static NAME_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^[a-zA-ZæøåÆØÅ\s]+$").unwrap());
