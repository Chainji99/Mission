use anyhow::Result;

use crate::config::{
    config_model::{Database, DotEnvyConfig, Server},
    stage::Stage,
};

pub fn load() -> Result<DotEnvyConfig> {
    dotenvy::dotenv().ok();

    let server = Server {
        port: std::env::var("SERVER_PORT")
            .expect("SERVER_PORT is valid")
            .parse()?,
        body_limit: std::env::var("SERVER_BODY_LIMIT")
            .expect("SERVER_BODY_LIMIT is valid")
            .parse()?,
        timeout: std::env::var("SERVER_TIMEOUT")
            .expect("SERVER_TIMEOUT is valid")
            .parse()?,
    };

    let database = Database {
        url: std::env::var("DATABASE_URL")
            .expect("DATABASE_URL is valid")
            .parse()?,
    };

    let secret = std::env::var("JWT_USER_SECRET")
        .expect("SECRET is valid")
        .parse()?;

    let config = DotEnvyConfig {
        server,
        database,
        secret, 
    };

    Ok(config)
}

pub fn get_stage() -> Stage {
    dotenvy::dotenv().ok();

    let stage_str = std::env::var("STAGE").unwrap_or("".to_string());
    Stage::try_form(&stage_str).unwrap_or_default()
}


pub fn get_jwt_env() -> JwtEnv {
    let dotenvy_env = match load() {
        Ok(env) => env,
        Err(_) => panic!("Failed to load dotenvy env"),
        std::result::Result::Ok(env) => env,
    };
    
}

pub fn get_cloudinary_env() -> CloudinaryEnv {
    dotenvy::dotenv().ok();
    Ok(CloudinaryEnv {
        cloud_name: env::var("CLOUDINARY_CLOUD_NAME")?,
        api_key: env::var("CLOUDINARY_API_KEY")?,
        api_secret: env::var("CLOUDINARY_API_SECRET")?, 
    })
}    