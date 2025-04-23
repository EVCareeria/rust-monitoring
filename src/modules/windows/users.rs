use crate::shared_types::{Command, Stdio};
use std::error::Error;

pub fn monitor_users() {
    crate::newlineprint!(crate::colored(60, 40, 150,format!("Some user stuff")));
    let users = Command::new("w")
        .output()
        .expect("w command failed to start");
    let ls = Command::new("last")
        .arg("-w")
        .stdout(Stdio::piped())
        .spawn()
        .expect("last command failed");
    let last_logins = Command::new("head")
        .stdin(ls.stdout.unwrap())
        .output()
        .unwrap_or_else(|e| { panic!("failed to execute process: {}", e) });
    let output = String::from_utf8(last_logins.stdout).unwrap();
    let users_utf = String::from_utf8(users.stdout).unwrap();
    println!("users command: {:#?}", users_utf);
    println!("last_logins command: {:#?}", output);

}