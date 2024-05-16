#![allow(dead_code)]
use std::env;

use dotenvy::dotenv;

// 使用 trait 来优化代码
trait FormEnv {
    fn form_env() -> Self;
}

#[derive(Debug)]
struct Server {
    host: String,
    port: u16,
    log_level: String,
}

impl FormEnv for Server {
    fn form_env() -> Self {
        let host = env::var("HOST").expect("HOST is not set in .env.local file");
        let port = env::var("PORT").expect("PORT is not set in .env.local file");
        let log_level = env::var("LOG_LEVEL").expect("logger_level is not set in .env.local file");

        Self {
            host,
            port: port.parse::<u16>().unwrap(),
            log_level,
        }
    }
}

#[derive(Debug)]
struct Db {
    host: String,
    port: u16,
    username: String,
    password: String,
    dbname: String,
}

impl FormEnv for Db {
    fn form_env() -> Self {
        let host = env::var("DATABASE_HOST").expect("DATABASE_HOST is not set in .env.local file");
        let port = env::var("DATABASE_PORT").expect("DATABASE_PORT is not set in .env.local file");
        let username =
            env::var("DATABASE_USERNAME").expect("DATABASE_USERNAME is not set in .env.local file");
        let password =
            env::var("DATABASE_PASSWORD").expect("DATABASE_PASSWORD is not set in .env.local file");
        let dbname =
            env::var("DATABASE_DBNAME").expect("DATABASE_DBNAME is not set in .env.local file");

        Self {
            host,
            port: port.parse::<u16>().unwrap(),
            username,
            password,
            dbname,
        }
    }
}

#[derive(Debug)]
struct Configs {
    server: Server,
    db: Db,
}

impl FormEnv for Configs {
    fn form_env() -> Self {
        dotenv().ok();
        let server = Server::form_env();
        let db = Db::form_env();

        Self { server, db }
    }
}

#[cfg(test)]
mod text_dotenvy {
    use super::*;

    fn set_env() {
        env::set_var("HOST", "127.0.0.1");
        env::set_var("PORT", "8000");
        env::set_var("LOG_LEVEL", "DEBUG");
        env::set_var("DATABASE_HOST", "127.0.0.1");
        env::set_var("DATABASE_PORT", "3306");
        env::set_var("DATABASE_USERNAME", "root");
        env::set_var("DATABASE_PASSWORD", "root");
        env::set_var("DATABASE_DBNAME", "nguc");
    }

    #[test]
    fn test_configs_server() {
        set_env();
        let config = Configs::form_env();
        assert_eq!(config.server.host, "127.0.0.1");
        assert_eq!(config.server.port, 8000);
        assert_eq!(config.server.log_level, "DEBUG");
    }
    #[test]
    fn test_configs_db() {
        set_env();
        let config = Configs::form_env();
        assert_eq!(config.db.host, "127.0.0.1");
        assert_eq!(config.db.port, 3306);
        assert_eq!(config.db.username, "root");
        assert_eq!(config.db.password, "root");
        assert_eq!(config.db.dbname, "nguc");
    }
}
