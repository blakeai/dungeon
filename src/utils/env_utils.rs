use std::env;
use std::path::Path;

use dotenv::dotenv;

pub(crate) fn load_env(variable_name: String) -> String {
    dotenv().ok();
    let value = env::var(variable_name).unwrap();
    return value;
}

pub(crate) fn load_filename(variable_name: String) -> String {
    let filename = load_env(variable_name);
    let path_buf = Path::new("resources").join(filename);
    return path_buf.to_str().unwrap().to_string();
}