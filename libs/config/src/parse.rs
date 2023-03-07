use crate::{
    config::{auth_config::AuthConfig, ClusterConfig},
    error::{self, ConfigResult},
};
use serde::de::DeserializeOwned;

#[derive(serde::Deserialize)]
struct DefaultConfig {
    kind: String,
}

pub fn parse(config_yaml: &str) -> ConfigResult<ClusterConfig> {
    let default_config = parse_yaml::<DefaultConfig>(config_yaml)?;

    match default_config.kind.as_str() {
        "Auth" => {
            let config = parse_yaml::<AuthConfig>(config_yaml)?;
            Ok(ClusterConfig::Auth(config))
        }
        _ => {
            let msg = format!("Unknown kind specified '{}'", default_config.kind);
            Err(error::new(msg))
        }
    }
}

fn parse_yaml<T>(config: &str) -> ConfigResult<T>
where
    T: DeserializeOwned,
{
    match serde_yaml::from_str::<T>(config) {
        Ok(conf) => Ok(conf),
        Err(err) => Err(error::new(err.to_string())),
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn can_parse_correct_config() {
        let config = parse(
            "
kind: Auth
server: localhost:9999
user:
    username: bob
    password: thebuilder
",
        );

        assert!(config.is_ok())
    }

    #[test]
    fn can_parse_incorrect_config() {
        let config = parse(
            "
something
",
        );

        assert!(config.is_err());
    }

    #[test]
    fn can_parse_unknown_kind() {
        let config = parse(
            "
kind: SomeUnknownKind
",
        );

        assert!(config.is_err());
    }
}
