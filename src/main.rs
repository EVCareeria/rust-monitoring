use crate::shared_types::*;

mod constants;
mod shared_types;
mod network_packets;
mod modules;

struct response_data {
    
}



#[macro_export]
macro_rules! newlineprint {
    ($message:expr) => {
        println!("\n{}", $message);
    };
}

fn main() {
    'outer: loop {
        let mut sys = System::new_all();
    
        print!("Print system information \n");
        print!("m for memory \ns for system\nn for network\nc for cpu\nd for disk data\n");
        let mut monitor_input = String::new();
    
        std::io::stdin()
            .read_line(&mut monitor_input)
            .expect("Failed to read line");
    
        match monitor_input.trim() {
            "m" => memory_info(&mut sys),
            "c" => modules::cpu::cpu_info(&mut sys),
            "s" => system_meta_data(),
            "n" => network_data(),
            "d" => modules::disk::disk_data(),
            _ => newlineprint!(format!("\nInvalid argument: {}", monitor_input)),
        }

        sleep(constants::DELAY);
        println!("\n")
       
    }

}

fn memory_info(sys: &mut System) {
    
    // Please note that we use "new_all" to ensure that all lists of
    // CPUs and processes are filled!
    
    // First we update all information of our `System` struct.
    sys.refresh_all();
    
    newlineprint!("=> system:");
    // RAM and swap information:
    let mut total_memory = sys.total_memory().to_string();
    newlineprint!(format!("total memory: {} gigabytes", bytes_to_gigabytes(sys.total_memory())));
    newlineprint!(format!("used memory : {} gigabytes", bytes_to_gigabytes(sys.used_memory())));
    newlineprint!(format!("free memory : {} gigabytes", bytes_to_gigabytes(sys.free_memory())));
    newlineprint!(format!("available memory : {} gigabytes", bytes_to_gigabytes(sys.available_memory())));
    newlineprint!(format!("total swap  : {} gigabytes", bytes_to_gigabytes(sys.total_swap())));
    newlineprint!(format!("used swap   : {} gigabytes", bytes_to_gigabytes(sys.used_swap())));

}

fn system_meta_data() {
 // Display system information:
    newlineprint!("=> system info:");

    println!("System name:             {:?}", System::name());
    println!("System kernel version:   {:?}", System::kernel_version());
    println!("System OS version:       {:?}", System::os_version());
    println!("System host name:        {:?}", System::host_name());
    
}

fn network_data() {
    let networks = Networks::new_with_refreshed_list();
    newlineprint!("=> network:");

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

pub fn bytes_to_gigabytes(value: u64) -> f32 {
    value as f32 / constants::BYTES_IN_GB as f32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn convert_to_gb() {
        let gb_8: u64 = 8308269056;
        let result = bytes_to_gigabytes(gb_8);
        assert_eq!(result, 8.3082695);
    }
}
