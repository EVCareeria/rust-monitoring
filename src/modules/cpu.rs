use crate::shared_types::System;

struct cpu_data {
    thread: String,
    usage: f32,
    filesystem: String,
}

pub fn cpu_info(sys: &mut System) {
    let mut cpu_info: Vec<cpu_data> = Vec::new();
    // Wait till our cpu data is fetched
    std::thread::sleep(sysinfo::MINIMUM_CPU_UPDATE_INTERVAL);
    // Update cpu data
    sys.refresh_cpu_all();
    sys.refresh_cpu_usage();

    crate::newlineprint!("=> cpu:");

    let cpus = sys.cpus();
    println!("NB CPUs: {}", cpus.len());

    for core in cpus {
        println!("Core {}  name", core.name());
        println!("Core {} % in use", core.cpu_usage());
        println!("Core {}  frequency", core.frequency());
    }
}
