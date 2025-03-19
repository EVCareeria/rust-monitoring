use crate::shared_types::Disk;
use crate::{bytes_to_gigabytes, newlineprint};
use sysinfo::Disks;

#[derive(Debug, PartialEq)]
pub struct disk_data {
    name: String,
    available_memory: f32,
    total_memory: f32,
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
            available_memory: bytes_to_gigabytes(disk.available_space()),
            total_memory: bytes_to_gigabytes(disk.total_space()),
            filesystem: match filesystem {
                Ok(fs) => fs,
                Err(osstr) => "Could not convert to string".to_string(),
            },
        }
    }

    fn print_data(&self) {
        newlineprint!(crate::colored(32, 111, 255,format!("Disk information: [ \n Name: {} \n Total: {:.1} gb \n Available: {:.1} gb \n Filesystem: {} \n ] ", 
        self.name, self.total_memory, self.available_memory, self.filesystem)));
    }
}

pub fn disk_data() {
    let mut disk_info: Vec<disk_data> = Vec::new();

    let disks = Disks::new_with_refreshed_list();
    for disk in disks.list() {
        println!("disk: {:?}", disk);
        let data = disk_data::create_new(disk);

        if disk_info.contains(&data) {
            continue;
        }
        data.print_data();

        disk_info.push(data);
    }
}

pub fn return_disk() -> Vec<disk_data> {
    let mut disk_info: Vec<disk_data> = Vec::new();

    let disks: Disks = Disks::new_with_refreshed_list();
    for disk in disks.list() {
        let data: disk_data = disk_data::create_new(disk);

        if disk_info.contains(&data) {
            continue;
        }

        disk_info.push(data);
    }
    disk_info
} 