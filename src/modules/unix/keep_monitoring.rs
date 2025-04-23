use crate::memory::memory_data;
use crate::shared_types::{System, Duration, sleep};
use crate::modules;

pub fn keep_monitoring(sys: &mut System){

    loop {
        
        let memory_data: memory_data = crate::memory::return_memory(sys);
        let cpu_data = crate::cpu::return_cpu(sys);
        let disk_data = crate::disk::return_disk();
        crate::newlineprint!(50, 40, 120,format!("{:#?}", memory_data));
        crate::newlineprint!(90, 70, 100,format!("{:#?}", cpu_data));
        crate::newlineprint!(32, 111, 255,format!("{:#?}", disk_data));
        //println!("{:?}", cpu_data);
        //println!("{:?}", disk_data);
        
        sleep(Duration::from_secs(5));
    }
    
    
}