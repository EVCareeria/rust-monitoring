use crate::shared_types::System;
use crate::{round_to_single_digit};

#[derive(Debug)]
pub struct cpu_data {
    core: String,
    usage: f32,
    name: String,
}

impl cpu_data {
    fn create_new(sys: &mut System) -> &mut System {
        // Wait till our cpu data is fetched
        std::thread::sleep(sysinfo::MINIMUM_CPU_UPDATE_INTERVAL);
        // Update cpu data
        sys.refresh_cpu_all();
        sys.refresh_cpu_usage();
        sys
    }

    fn cpu_monitor(sys: &mut System) {
        crate::newlineprint!("=> cpu:");

        let cpus = sys.cpus();
        println!("NB CPUs: {}", cpus.len());

        for core in cpus {
            println!("Core {}  name", core.name());
            println!("Core {} % in use", core.cpu_usage());
            println!("Core {}  frequency", core.frequency());
        }
    }
}

pub fn cpu_info(sys: &mut System) {
    let sys = cpu_data::create_new(sys);
    cpu_data::cpu_monitor(sys);
}

pub fn return_cpu(sys: &mut System) -> Vec<cpu_data> {
    let mut cpu_info: Vec<cpu_data> = Vec::new();

    let cpus = sys.cpus();
    println!("NB CPUs: {}", cpus.len());

    for core in cpus {
        let data = cpu_data {
            name: core.name().to_owned().to_string(),
            usage: round_to_single_digit(core.cpu_usage() as f32),
            core: round_to_single_digit(core.frequency() as f32).to_owned().to_string()
        };
        cpu_info.push(data);
    }
    cpu_info
}
