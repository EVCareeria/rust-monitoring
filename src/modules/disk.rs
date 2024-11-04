use crate::shared_types::Disk;
use crate::{bytes_to_gigabytes, newlineprint};

#[derive(Debug, PartialEq, Eq)]
struct disk_data {
    name: String,
    available_memory: u64,
    total_memory: u64,
    filesystem: String,
}

impl disk_data {
    fn create_new(disk: &Disk) -> disk_data {
        let filesystem = disk.file_system().to_owned().into_string();
        let name = disk.name().to_owned().into_string();
        disk_data {
            name: match name {
                Ok(fs) => fs,
                Err(osstr) => "Could not convert to string".to_string(),
            },
            available_memory: disk.available_space(),
            total_memory: disk.total_space(),
            filesystem: match filesystem {
                Ok(fs) => fs,
                Err(osstr) => "Could not convert to string".to_string(),
            },
        }
    }

    fn print_data(&self) {
        newlineprint!(crate::colored(32, 111, 255,format!("Disk information: [ \n Name: {} \n Total: {} gb \n Available: {} gb \n Filesystem: {} \n ] ", 
        self.name, bytes_to_gigabytes(self.total_memory),  bytes_to_gigabytes(self.available_memory), self.filesystem)));
    }
}

pub fn disk_data() {
    let mut disk_info: Vec<disk_data> = Vec::new();
    use sysinfo::Disks;

    let disks = Disks::new_with_refreshed_list();
    for disk in disks.list() {
        let data = disk_data::create_new(disk);

        if disk_info.contains(&data) {
            continue;
        }
        data.print_data();

        disk_info.push(data);
    }
}
