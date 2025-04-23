use std::fmt::{Debug, Display};
use crate::shared_types::*;
mod constants;
mod modules;
mod network_packets;
mod shared_types;

#[cfg(target_os = "windows")]
use modules::windows::{cpu, memory, keep_monitoring, users, disk};

#[cfg(unix)]
use modules::unix::{cpu, memory, keep_monitoring, users, disk};



#[macro_export]
macro_rules! newlineprint {
    ($r:expr, $g:expr, $b:expr, $message:expr) => {
        println!(
            "\n{}",
            crate::colored($r, $g, $b, format!("\n{}", $message))
        );
    };
    // Match if no color values given
    ($message:expr) => {
        println!(
            "\n{}",
            crate::colored(255, 255, 255, format!("\n{}", $message)) // default RGB (white)
        );
    };
}

pub fn colored<T: std::fmt::Display>(r: i32, g: i32, b: i32, text: T) -> String {
    format!("\x1B[38;2;{};{};{}m{}\x1B[0m", r, g, b, text)
}

pub fn bytes_to_gigabytes(value: u64) -> f32 {
    value as f32 / constants::BYTES_IN_GB as f32
}

pub fn round_to_single_digit(val: f32) -> f32 {
    (val * 10.0).round() / 10.0 
}

pub fn system_info() {
    'outer: loop {
        let mut sys = System::new_all();
        
        let load_avg = System::load_average();
        println!(
            "one minute: {}%, five minutes: {}%, fifteen minutes: {}%",
            load_avg.one,
            load_avg.five,
            load_avg.fifteen,
        );


        print!("Print system information \n");
        print!("k for keeep monitoring\nm for memory \ns for system\nn for network\nc for cpu\nd for disk data\nt for temperature\nu for users data\nx for clearing the screen\n");
        let mut monitor_input = String::new();

        std::io::stdin()
            .read_line(&mut monitor_input)
            .expect("Failed to read line");

        let (tx, rx) = mpsc::channel();

        spawn(move || {
            let val = String::from("hi");
            tx.send(val).unwrap();
        });
        
        #[cfg(target_os = "windows")]{
            match monitor_input.trim() {
            
                "k" => keep_monitoring::keep_monitoring(&mut sys),
                "m" => memory::memory_info(&mut sys),
                "t" => component_data(),
                "c" => cpu::cpu_info(&mut sys),
                "s" => system_meta_data(),
                "n" => network_data(),
                "d" => disk::disk_data(),
                "u" => users::monitor_users(),
                "x" => println!("\r\x1b[2J\r\x1b[H"),
                _ => newlineprint!(format!("\nInvalid argument: {}", monitor_input)),
            }
        }

        #[cfg(unix)]{
            match monitor_input.trim() {
            
                "k" => keep_monitoring::keep_monitoring(&mut sys),
                "m" => memory::memory_info(&mut sys),
                "t" => component_data(),
                "c" => cpu::cpu_info(&mut sys),
                "s" => system_meta_data(),
                "n" => network_data(),
                "d" => disk::disk_data(),
                "u" => users::monitor_users(),
                "x" => println!("\r\x1b[2J\r\x1b[H"),
                _ => newlineprint!(format!("\nInvalid argument: {}", monitor_input)),
            }
        }
    
        sleep(constants::DELAY);
        println!("\n")
    }
}

fn component_data() {

    let components = Components::new_with_refreshed_list();
    for component in &components {
        println!("{} {:?}°C", component.label(), component.temperature());
    }
}

fn system_meta_data() {
    // Display system information:
    newlineprint!(0,0,0, "=> system info:");

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn convert_to_gb() {
        let result = bytes_to_gigabytes(8308269056);
        assert_eq!(result, 8.3082695);
    }
}
