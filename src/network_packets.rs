use pcap::Device;
use sysinfo::Networks;

pub fn packets() {
    let mut cap = Device::lookup().unwrap().unwrap().open().unwrap();

    while let Ok(packet) = cap.next_packet() {
        println!("received packet! {:?}", packet);
    }


    let networks = Networks::new_with_refreshed_list();
    for (interface_name, network) in &networks {
        println!("[{interface_name}] {network:?}");
    }
}
