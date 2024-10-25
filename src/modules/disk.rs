use crate::{bytes_to_gigabytes, newlineprint};

#[derive(Debug, PartialEq, Eq)]
struct disk_data {
    name: String,
    available_memory: u64,
    total_memory: u64,
    filesystem: String,
}

pub fn disk_data() {
    let mut disk_info: Vec<disk_data> = Vec::new();
    use sysinfo::Disks;

    let disks = Disks::new_with_refreshed_list();
    for disk in disks.list() {
        let filesystem = disk.file_system().to_owned().into_string();
        let name = disk.name().to_owned().into_string();

        let data = disk_data {
            name: match name {Ok(fs) => fs, Err(osstr) => "Could not convert to string".to_string()},
            available_memory:disk.available_space(),
            total_memory: disk.total_space(),
            filesystem: match filesystem {Ok(fs) => fs, Err(osstr) => "Could not convert to string".to_string()}
            };
            
        if disk_info.contains(&data) {continue}

        newlineprint!(format!("Disk information: Name {} Total {} gb, Available {} gb, Filesystem {}", 
                    data.name, bytes_to_gigabytes(data.total_memory),  bytes_to_gigabytes(data.available_memory), data.filesystem));
        disk_info.push(data);
    }
}