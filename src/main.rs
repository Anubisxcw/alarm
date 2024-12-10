mod config;
mod dbo;
mod qywx;
mod alertmanager;

use crate::config::load_or_init_config;
use crate::dbo::{get_all_data, init_db};
use crate::qywx::get_wechat_token_with_proxy;
use duckdb:: Connection;
use tracing::Level;

use tracing_subscriber:: FmtSubscriber;

#[tokio::main]
async fn main() {
    setup_tracing(Level::INFO);
    tracing::info!("Server starting up");
    tracing::info!("Reading config file from config.toml");
    let config = match load_or_init_config() {
        Ok(v) => v,
        Err(err) => {
            match err {
                config::ConfigError::IoError(err) => {
                    tracing::error!("Error reading config file: {}", err)
                }
                config::ConfigError::InvalidConfig(err) => {
                    tracing::error!("Invalid config file: {}", err);
                }
            }
            return;
        }
    };
    tracing::info!("Loaded config: {:?}", config);
    let conn = Connection::open(config.get_db_file_name()).expect("Failed to open database");
    init_db(&conn);
    let cluster_info = get_all_data(&conn).expect("faild to get all data");
    for cluster in cluster_info {
        tracing::info!("{:?} ", cluster);
    }
    get_wechat_token_with_proxy(&config).await.expect("get token error");
}


fn setup_tracing(level: Level) {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(level)
        .finish();

    tracing::subscriber::set_global_default(subscriber)
        .expect("Failed to set global subscriber");
}