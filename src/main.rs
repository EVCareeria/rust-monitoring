use std::sync::{Arc, Mutex};

use sysinfo::{
    Components, Disks, Networks, System,
};

fn main() {
    let sys = Arc::new(Mutex::new(System::new_all()));

    let sys_clone = Arc::clone(&sys);
    std::thread::spawn(move || {
        let mut sys = sys_clone.lock().unwrap(); // Lock the mutex to access sys
        memory_info(&mut sys);
    });
    let mut sys = sys.lock().unwrap();
    system_meta_data();
    cpu_info(&mut sys);
    network_data();
}


fn memory_info(sys: &mut System) {
    
    // Please note that we use "new_all" to ensure that all lists of
    // CPUs and processes are filled!
    
    // First we update all information of our `System` struct.
    sys.refresh_all();
    
    println!("=> system:");
    // RAM and swap information:
    let mut total_memory = sys.total_memory().to_string();
    let memory_parts = total_memory.split_at(2);
    let total_memory_after_comma: String = memory_parts.1.chars().into_iter().take(3).collect();
    println!("total memory: {0},{1} gigabytes", memory_parts.0, total_memory_after_comma);
    println!("used memory : {} gigabytes", sys.used_memory());
    println!("total swap  : {} gigabytes", sys.total_swap());
    println!("used swap   : {} gigabytes", sys.used_swap());
        
}

fn cpu_info(sys: &mut System) {
    // Wait till our cpu data is fetched
    std::thread::sleep(sysinfo::MINIMUM_CPU_UPDATE_INTERVAL);
    // Update cpu data
    sys.refresh_cpu_usage();

    let cpus = sys.cpus();
    println!("NB CPUs: {}", cpus.len());
    
    for core in cpus {
        println!("Core {0} {1} % in use", core.name(), core.cpu_usage().round());
    }

}

fn system_meta_data() {
 // Display system information:
    println!("System name:             {:?}", System::name());
    println!("System kernel version:   {:?}", System::kernel_version());
    println!("System OS version:       {:?}", System::os_version());
    println!("System host name:        {:?}", System::host_name());
    
}

fn network_data() {
    let networks = Networks::new_with_refreshed_list();
    for (interface_name, data) in &networks {
    println!(
        "{interface_name}: {} B (down) / {} B (up)",
        data.total_received(),
        data.total_transmitted(),
    );
    // If you want the amount of data received/transmitted since last call
    // to `Networks::refresh`, use `received`/`transmitted`.
    }
}