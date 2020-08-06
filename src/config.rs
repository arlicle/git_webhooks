use std::fs;
use json5;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};


#[derive(Serialize,Deserialize,Debug)]
pub struct Project {
    name:Option<String>,
    cwd:Option<String>,
    command:Option<Value>,
    branch:Option<String>,
    secret:Option<String>,
}



#[derive(Serialize,Deserialize,Debug)]
pub struct Config {
    pub secret:Option<String>,
    pub command:Option<Value>,
    pub projects:Option<Vec<Project>>
}

impl Config {

    /// Load project config
    pub fn new() -> Self {
        let config_file = "config.json5";
        match fs::read_to_string(config_file) {
            Ok(v) => {
                match json5::from_str::<Config>(&v) {
                    Ok(v) => {
                        return v;
                    }
                    Err(e) => {
                        println!("Parse json file {} error : {:?}", config_file, e);
                    }
                }
            }
            Err(_) => (),
        };

        Config {
            secret:None,
            command:None,
            projects:None,
        }
    }
}