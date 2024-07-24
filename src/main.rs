#[macro_use]
extern crate lazy_static;

use axum::Router;
use tokio::net::TcpListener;
use tracing::Level;
use tracing_subscriber::FmtSubscriber;
use std::str::FromStr;

use crate::config::environment::{EnvVar};
use crate::controller::class_controller;
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
        .nest("/classes", class_controller::router());

    let hostname = create_hostname();
    let listener = TcpListener::bind(hostname).await.unwrap();

    axum::serve(listener, router).await.unwrap();
}

fn create_hostname() -> String {
    let port = getenv(EnvVar::Port);
    return format!("0.0.0.0:{}", port);
}

fn setup_logging() {
    let log_level_env = getenv(EnvVar::LogLevel);
    let level = Level::from_str(&log_level_env).unwrap_or(Level::INFO);
    tracing::subscriber::set_global_default(
        FmtSubscriber::builder()
            .with_max_level(level)
            .finish()
    ).expect("setting default subscriber failed");
}