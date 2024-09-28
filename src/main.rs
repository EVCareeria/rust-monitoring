use sysinfo::{
    Components, Disks, Networks, System,
};

fn main() {
    println!("Hello, world!");
    system_info();
}

fn system_info() {
    
    // Please note that we use "new_all" to ensure that all lists of
    // CPUs and processes are filled!
    let mut sys = System::new_all();
    
    // First we update all information of our `System` struct.
    sys.refresh_all();
    
    println!("=> system:");
    // RAM and swap information:
    let mut total_memory = sys.total_memory().to_string();
    let (first, second) = total_memory.split_at(2);
    let total_memory_after_comma: String = second.chars().into_iter().take(3).collect();
    println!("total memory: {0},{1} gigabytes", first, total_memory_after_comma);
    println!("used memory : {} gigabytes", sys.used_memory());
    println!("total swap  : {} gigabytes", sys.total_swap());
    println!("used swap   : {} gigabytes", sys.used_swap());
    
    // Display system information:
    println!("System name:             {:?}", System::name());
    println!("System kernel version:   {:?}", System::kernel_version());
    println!("System OS version:       {:?}", System::os_version());
    println!("System host name:        {:?}", System::host_name());
    
    // Number of CPUs:
    println!("NB CPUs: {}", sys.cpus().len());
    
}