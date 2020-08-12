use std::process::Command;
use execute::{Execute,command};


use std::thread;
use crossbeam_channel::{unbounded,Sender,Receiver};

pub struct Task {
    pub sender:Sender<Vec<String>>,

}


impl Task {
    pub fn run() -> Task {
        let (tx, rx) = unbounded();
        let tx1 = tx.clone();

        thread::spawn(move || {
            for received_commands in rx {
                println!("received_command: {:?}", received_commands);
                thread::spawn(move || {
                    for command in received_commands {
                        Task::run_command(command);
                    }
                });
            }
        });

        Task {
            sender:tx
        }
    }

    pub fn send(&self, command:Vec<String>) {
        self.sender.send(command).unwrap();
    }

    fn run_command(command:String) {
        println!("command {:?}", command);
        let s: Vec<&str> = command.split(" ").collect();
//        let mut echo_hello = Command::new(s[0]);

        let mut aaa = Command::new("pwd").output().unwrap();
        let request_body = std::str::from_utf8(&aaa.stdout).unwrap();
        println!("output: {}", request_body);

        let mut aaa = Command::new("cd").arg("/www/hekou_bigdata").output().unwrap();
        let request_body = std::str::from_utf8(&aaa.stdout).unwrap();
        println!("output: {}", request_body);

//        if s.len() > 1 {
//            for arg in &s[1..] {
//                echo_hello.arg(arg);
//            }
//        }
//        let aaa = echo_hello.output().expect("failed to execute process");
//        let request_body = std::str::from_utf8(&aaa.stdout).unwrap();
        println!("output: {}", request_body);

    }
}