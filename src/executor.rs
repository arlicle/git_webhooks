use std::process::Command;
use std::thread;
use std::sync::mpsc;

pub struct Task {
    pub sender:mpsc::Sender<String>,

}


impl Task {
    pub fn run() -> Self {
        let (tx, rx):(mpsc::Sender<String>, mpsc::Receiver<String>) = mpsc::channel();
        let tx1 = mpsc::Sender::clone(&tx);
        let tx2 = mpsc::Sender::clone(&tx);

        thread::spawn(move || {
            for received_command in rx {
                thread::spawn(move || {
                    Task::run_command(received_command);
                });
            }
        });

        Task {
            sender:tx2
        }
    }

    pub fn send(&self, command:&str) {
        self.sender.send(command.to_string()).unwrap();
    }

    fn run_command(command:String) {
        let s: Vec<&str> = command.split(" ").collect();
        let mut echo_hello = Command::new(s[0]);
        if s.len() > 1 {
            echo_hello.args(&s[1..]);
        }
        let aaa = echo_hello.output().expect("failed to execute process");
        let request_body = std::str::from_utf8(&aaa.stdout).unwrap();
        println!("{}", request_body);

    }
}