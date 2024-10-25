pub use sysinfo::{
    Components, Disks, Networks, System,
};
pub use std::{sync::{mpsc::{self, Receiver, Sender}, Arc, Mutex}, thread::sleep, time::Duration};
