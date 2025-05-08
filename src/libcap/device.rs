use pcap::Device;

pub fn select_capture_device() -> Device {
    use pcap::{Device, Error};
    let device_list = Device::list().unwrap();
    let selected_device = device_list
        .into_iter()
        .filter(|d| {
            d.name.contains("eth") || d.name.contains("wlan") || d.name.contains("en")
        })
        .next()
        .expect("No network interface found");

    println!("Selected device : {}", selected_device.name);
    selected_device
}

pub fn list_net_interfaces() -> Vec<Device> {
    println!("------------- Available network interfaces -------------");
    Device::list().unwrap()
        .into_iter()
        .map(|x| {
            if x.name.contains("eth") || x.name.contains("wlan") || x.name.contains("en") {
                println!("(Recommended) {}", x.name)
            } else {
                println!("{}", x.name)
            }
            x
        })
        .collect()
    
}