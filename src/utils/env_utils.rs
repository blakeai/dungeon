use std::env;
use std::path::Path;

use dotenv::from_filename;

pub(crate) fn load_env(variable_name: String) -> String {
    let env_file = if cfg!(test) { ".env.test" } else { ".env" };
    from_filename(env_file).ok();
    let value = env::var(variable_name).unwrap();
    return value;
}

pub(crate) fn load_filename(variable_name: String) -> String {
    let filename = load_env(variable_name);
    return Path::new(&filename).to_str().unwrap().to_string();
}