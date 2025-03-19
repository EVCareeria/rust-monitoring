use crate::shared_types::System;
use crate::{bytes_to_gigabytes, newlineprint};

use super::cpu::cpu_data;

#[derive(Debug)]
pub struct memory_data {
    total_memory: f32,
    used_memory: f32,
    free_memory: f32,
    available_memory: f32,
    total_swap: f32,
    used_swap: f32,
}

impl memory_data {
    fn create_new(sys: &mut System) -> memory_data {
        memory_data {
            total_memory: bytes_to_gigabytes(sys.total_memory()),
            used_memory: bytes_to_gigabytes(sys.used_memory()),
            free_memory: bytes_to_gigabytes(sys.free_memory()),
            available_memory: bytes_to_gigabytes(sys.available_memory()),
            total_swap: bytes_to_gigabytes(sys.total_swap()),
            used_swap: bytes_to_gigabytes(sys.used_swap()),
        }
    }

    fn print_memory(&self) {
        println!("{:#?}", self);
    }
}

pub fn memory_info(sys: &mut System) {
    // Please note that we use "new_all" to ensure that all lists of
    // CPUs and processes are filled!

    // First we update all information of our `System` struct.
    sys.refresh_all();

    let memory = memory_data::create_new(sys);
    memory.print_memory();
}

pub fn return_memory(sys: &mut System) -> memory_data {
    sys.refresh_all();

    let memory = memory_data::create_new(sys);
    memory
}
