use std::fs::File;
use std::io::{Read, Write};
use std::path::PathBuf;
use toml::value::Table;
use serde::{Deserialize, Serialize};
use toml::Value;

#[derive(Deserialize, Serialize)]
pub struct Config {
    pub cycle: u64,
    pub discord: Discord,
    pub triggers: Table,
}

#[derive(Deserialize, Serialize)]
pub struct Discord {
    pub details: String,
    pub status: String,
}

pub struct Trigger {
    pub process_name: String,
    pub window_name: Option<String>,
}

impl Config {
    fn default() -> Self {
        Config {
            cycle: 3000,
            discord: Discord {
                details: String::from("https://github.com/Jhyub/rphide"),
                status: String::from("Hiding discord activity"),
            },
            triggers: Default::default(),
        }
    }

    fn config_path() -> PathBuf {
        let mut buf = dirs::config_dir().unwrap();
        buf.push("rphide.toml");
        buf
    }

    pub fn load() -> Self {
        if !Config::config_path().exists() {
            Config::write(&Config::default());
            return Config::default()
        }
        let mut file = File::open(Config::config_path()).unwrap();
        let mut s = String::new();
        file.read_to_string(&mut s).unwrap();
        let config: Config = toml::from_str(s.as_str()).unwrap();
        config
    }

    pub fn write(config: &Self) {
        let toml = toml::to_string(config).unwrap();
        let mut file = File::create(Config::config_path()).unwrap();
        file.write_all(toml.as_bytes()).unwrap();
    }

    pub fn triggers(&self) -> Vec<Trigger> {
        fn value_to_string(value: Value) -> Option<String> {
            match value {
                Value::String(a) => if a.as_str() == "*" {
                    None
                } else {
                    Some(a)
                },
                _ => None
            }
        }

        let mut vec: Vec<Trigger> = vec![];
        for (k, v) in self.triggers.clone() {
            vec.push(Trigger { process_name: k, window_name: value_to_string(v) });
        }
        vec
    }
}