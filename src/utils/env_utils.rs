use std::env;
use std::path::Path;

use dotenv::from_filename;
use crate::config::environment::{Environment, EnvVar, ToKey};

pub(crate) fn getenv(env_var: EnvVar) -> String {
    let key = env_var.to_key();
    let env_file = env_file_name();
    from_filename(env_file).ok();
    return env::var(key).unwrap();
}

fn env_file_name() -> String {
    if cfg!(test) {
        format!(".env.{}", Environment::Test.to_key())
    } else {
        ".env".to_string()
    }
}