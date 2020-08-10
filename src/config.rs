use std::fs;
use std::collections::HashMap;
use json5;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};


#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub data: Value
}

impl Config {
    /// Load project config
    pub fn new() -> Self {
        let config_file = "config.json5";
        match fs::read_to_string(config_file) {
            Ok(v) => {
                match json5::from_str(&v) {
                    Ok(v) => {
                        return Config { data: v };
                    }
                    Err(e) => {
                        println!("Parse json file {} error : {:?}", config_file, e);
                    }
                }
            }
            Err(_) => (),
        };

        Config { data: Value::Null }
    }


    /// get project config data
    pub fn get_project_config_data(&self, project_name: &str, key: &str) -> Vec<String> {
        let mut vals: Vec<String> = Vec::new();
        self.get_vals(&mut vals, self.data.pointer(&format!("/projects/{}/{}", project_name, key)));

        // default is not inherit
        let mut is_inherit = false;
        if let Some(Value::Bool(v)) = self.data.pointer("/inherit") {
            is_inherit = *v;
        }

        if vals.is_empty() && is_inherit{
            self.get_vals(&mut vals, self.data.pointer(&format!("/{}", key)));
        }

        if vals.is_empty() {
            vals.push("".to_string());
        }
        vals
    }


    fn get_vals(&self, vals: &mut Vec<String>, value: Option<&Value>) {
        match value {
            Some(Value::Array(v)) => {
                for v1 in v {
                    vals.push(v1.as_str().unwrap().to_string());
                }
            }
            Some(Value::String(c)) => {
                vals.push(c.to_string());
            }
            Some(_) => (),
            None => ()
        }
    }

}