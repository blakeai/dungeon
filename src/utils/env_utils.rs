use std::env;
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
