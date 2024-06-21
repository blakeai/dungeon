#[macro_use]
extern crate lazy_static;

use axum::Router;
use tokio::net::TcpListener;
use tracing::Level;
use tracing_subscriber::FmtSubscriber;

use crate::config::environment::{EnvVar};
use crate::controller::character_controller;
use crate::utils::env_utils::getenv;

mod models;
mod utils;
mod service;
mod config;
mod controller;

#[tokio::main]
async fn main() {
    setup_logging();
    
    let router = Router::new()
        .nest("/characters", character_controller::router());
    
    let hostname = create_hostname();
    let listener = TcpListener::bind(hostname).await.unwrap();
    
    axum::serve(listener, router).await.unwrap();
}

fn create_hostname() -> String {
    let port = getenv(EnvVar::Port);
    return format!("0.0.0.0:{}", port);
}

fn setup_logging() {
    tracing::subscriber::set_global_default(
        FmtSubscriber::builder()
            .with_max_level(Level::INFO)
            .finish()
    ).expect("setting default subscriber failed");
}