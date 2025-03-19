use crate::shared_types::{System, Duration, sleep};
use crate::modules;

pub fn keep_monitoring(sys: &mut System){

    loop {
        
        let memory_data = modules::memory::return_memory(sys);
        let cpu_data = modules::cpu::return_cpu(sys);
        let disk_data = modules::disk::return_disk();
        crate::newlineprint!(50, 40, 120,format!("{:#?}", memory_data));
        crate::newlineprint!(90, 70, 100,format!("{:#?}", cpu_data));
        crate::newlineprint!(32, 111, 255,format!("{:#?}", disk_data));
        //println!("{:?}", cpu_data);
        //println!("{:?}", disk_data);
        
        sleep(Duration::from_secs(5));
    }
    
    
}