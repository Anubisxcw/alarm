use serde::{Deserialize, Serialize};
use std::{fs, io, path::Path};

#[derive(Serialize, Deserialize, Debug)]
pub struct AppConfig {
    db_file_name: String,
    db_user: String,
    db_password: String,
    db_name: String,
    corpid: String,
    corpsecret: String,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            db_file_name: "alarm.db".to_string(),
            db_user: "work".to_string(),
            db_password: "work".to_string(),
            db_name: "alarm".to_string(),
            corpid: "test".to_string(),
            corpsecret: "test".to_string(),

        }
    }
}
pub enum ConfigError {
    IoError(io::Error),
    InvalidConfig(toml::de::Error),
}

impl From<io::Error> for ConfigError {
    fn from(err: io::Error) -> Self {
        Self::IoError(err)
    }
}
impl From<toml::de::Error> for ConfigError {
    fn from(err: toml::de::Error) -> Self {
        Self::InvalidConfig(err)
    }
}
pub fn load_or_init_config() -> Result<AppConfig, ConfigError> {
    let config_file = Path::new("../config.toml");
    if config_file.exists() {
        let content = fs::read_to_string(config_file)?;
        let config: AppConfig = toml::from_str(&content)?;
        Ok(config)
    } else {
        let config = AppConfig::default();
        let content = toml::to_string(&config).unwrap();
        fs::write(config_file, content)?;
        Ok(config)
    }
}

impl AppConfig {
    pub fn get_db_file_name(&self) -> &str {
        &self.db_file_name
    }
    pub fn get_db_user(&self) -> &str {
        &self.db_user
    }
    pub fn get_db_password(&self) -> &str {
        &self.db_password
    }
    pub fn get_db_name(&self) -> &str {
        &self.db_name
    }

    pub fn get_corpid(&self) -> &str {
        &self.corpid
    }
    pub fn get_corpsecret(&self) -> &str {
        &self.corpsecret
    }
}
