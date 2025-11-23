pub mod db;
#[cfg(feature = "server")]
pub mod google;

pub mod models;
pub mod views;

mod config;
pub use config::UIConfig;
#[cfg(feature = "server")]
pub use config::Config;
