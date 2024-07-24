use std::env;
use dotenv::from_filename;
use crate::config::environment::{EnvVar, ToKey};

pub(crate) fn getenv(env_var: EnvVar) -> String {
    let key = env_var.to_key();
    let env_file = env_file_name();
    from_filename(env_file).ok();
    return env::var(&key).expect(&format!("env {} not found in .env file", &key));
}

fn env_file_name() -> String {
    let env_file = if cfg!(test) {
        ".env.test"
    } else {
        ".env"
    };
    return env_file.to_string();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_getenv() {
        let result = getenv(EnvVar::Port);
        assert_eq!(result, "3210");
    }

    #[test]
    fn test_env_file_name() {
        let result = env_file_name();
        assert_eq!(result, ".env.test");
    }
}
