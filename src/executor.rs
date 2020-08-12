use std::process::Command;
use execute::{Execute, command};


use std::thread;
use crossbeam_channel::{unbounded, Sender, Receiver};

pub struct Task {
    pub sender: Sender<Vec<String>>,

}


impl Task {
    pub fn run() -> Task {
        let (tx, rx) = unbounded();
        let tx1 = tx.clone();

        thread::spawn(move || {
            for received_commands in rx {
                let mut received_commands: Vec<String> = received_commands;
                thread::spawn(move || {
                    let cwd = received_commands.remove(0);
                    for command in received_commands {
                        Task::run_command(command, &cwd);
                    }
                });
            }
        });

        Task {
            sender: tx
        }
    }

    pub fn send(&self, command: Vec<String>) {
        self.sender.send(command).unwrap();
    }

    fn run_command(command_str: String, cwd: &String) {
        let s: Vec<&str> = command_str.split(" ").collect();
        let mut command = Command::new(s[0]);
        command.current_dir(cwd);

        if s.len() > 1 {
            command.args(&s[1..]);
        }

        let mut aaa = command.output().unwrap();
        let request_body = std::str::from_utf8(&aaa.stdout).unwrap();
        println!("output: {}", request_body);
    }
}