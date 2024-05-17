use std::env;

use config::{Config, File};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct App {
    pub host: String,
    pub port: String,
    pub echo: bool,
    pub log_level: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Database {
    pub host: String,
    pub port: String,
    pub username: String,
    pub password: String,
    pub dbname: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Configs {
    pub app: App,
    pub database: Database,
}

impl Configs {
    pub fn new() -> Self {
        let run_mode = env::var("RUN_MODE").unwrap_or_else(|_| "local".into());
        dbg!(&run_mode);

        let cfg = Config::builder()
            // 加载默认的配置
            .add_source(File::with_name("src/settings/files/default.toml"))
            // 加载运行环境(local/dev/prod)的配置
            .add_source(
                File::with_name(&format!("src/settings/files/{}.toml", run_mode)).required(false),
            )
            // 加载来自环境变量的配置, 以"app"为开头
            // E.g. `APP_DEBUG=1 ./target/app` would set the `debug` key
            // .add_source(Environment::with_prefix("app"))
            // You may also programmatically change settings
            // .set_override("database.url", "postgres://")?
            .build()
            .unwrap_or_else(|e| panic!("配置文件加载失败：{}", e));

        cfg.try_deserialize().unwrap()
    }
}

impl Default for Configs {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod test_config {
    use super::*;

    #[test]
    fn test_config_local() {
        env::set_var("RUN_MODE", "local");
        let cfg = Configs::new();
        let c_str = r#"{"app":{"host":"127.0.0.1","port":"8000","echo":true,"log_level":"DEBUG"},"database":{"host":"127.0.0.1","port":"3306","username":"root","password":"root","dbname":"app_local"}}"#;
        let cfg_str = serde_json::to_string(&cfg).unwrap();
        assert_eq!(cfg_str, c_str);
    }
    #[test]
    fn test_config_dev() {
        env::set_var("RUN_MODE", "dev");
        let cfg = Configs::new();
        let c_str = r#"{"app":{"host":"127.0.0.1","port":"8080","echo":true,"log_level":"DEBUG"},"database":{"host":"127.0.0.2","port":"3307","username":"root","password":"root","dbname":"app_dev"}}"#;
        let cfg_str = serde_json::to_string(&cfg).unwrap();
        assert_eq!(cfg_str, c_str);
    }
    #[test]
    fn test_config_prod() {
        env::set_var("RUN_MODE", "prod");
        let cfg = Configs::new();
        let c_str = r#"{"app":{"host":"127.0.0.1","port":"8000","echo":true,"log_level":"INFO"},"database":{"host":"127.0.0.3","port":"3308","username":"root","password":"root","dbname":"app_prod"}}"#;
        let cfg_str = serde_json::to_string(&cfg).unwrap();
        assert_eq!(cfg_str, c_str);
    }

    #[test]
    fn test_config_app() {
        env::set_var("RUN_MODE", "local");
        let cfg = Configs::new();
        assert_eq!(cfg.app.host, "127.0.0.1");
        assert_eq!(cfg.app.port, "8000");
        assert!(cfg.app.echo);
        assert_eq!(cfg.app.log_level, "DEBUG");
    }

    #[test]
    fn test_config_database() {
        env::set_var("RUN_MODE", "local");
        let cfg = Configs::new();
        assert_eq!(cfg.database.host, "127.0.0.1");
        assert_eq!(cfg.database.port, "3306");
        assert_eq!(cfg.database.username, "root");
        assert_eq!(cfg.database.password, "root");
        assert_eq!(cfg.database.dbname, "app_local");
    }
}
