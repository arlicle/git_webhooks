use std::process::Command;

pub struct Task {
    pub repository_name:String,
    pub branch:String,
    pub command: Vec<String>,
    pub cwd: String,
    pub created:u32
}


