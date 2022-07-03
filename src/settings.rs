use config::{Config, ConfigError, Environment, File, ConfigBuilder, builder::DefaultState};
use serde::{Deserialize};
use std::env;

#[derive(Debug, Deserialize)]
#[allow(unused)]
struct Twitter {
    api_key: String,
    api_key_secret: String,
    bearer_token: String,
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct Settings {
    debug: bool,
    twitter: Twitter,
}


impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        return get_config(get_run_mode());
    }
}

fn get_run_mode() -> String{
    let args: Vec<String> = env::args().collect();
        
    let mut run_mode = "production";    
    if args.len() > 1{
        run_mode = &args[1];
    }
    if run_mode == "dev"{ 
        println!("run_mode: {:?}", run_mode);
    }
    return run_mode.to_string();
}

fn get_config(app_run_mode: String) -> Result<Settings, ConfigError>{
    let s;

    if app_run_mode == "dev"{
        s = Config::builder()
            // Merging in the "local" configuration file
            .add_source(File::with_name("src/config/local.toml").required(false))
            // Add in settings from the environment (with a prefix of APP) - DON'T KNOW IF IT'S NECESSARY
            // Eg.. `APP_DEBUG=1 ./target/app` would set the `debug` key
            .add_source(Environment::with_prefix("app"))
            .build()?;           
            
            // Now that we're done, let's access our configuration
            println!("debug: {:?}", s.get_bool("debug"));
    }else {
        s = Config::builder()
            // Merging in the "production" configuration file
            .add_source(File::with_name("src/config/production.toml"))
            // Add in settings from the environment (with a prefix of APP)
            // Eg.. `APP_DEBUG=1 ./target/app` would set the `debug` key
            .add_source(Environment::with_prefix("app"))
            .build()?;
    }
    s.try_deserialize()
}