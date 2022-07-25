use lazy_static::lazy_static;
use local_ip_address::list_afinet_netifas;
use std::net::{IpAddr, Ipv4Addr};
use tokio::net::UdpSocket;

lazy_static! {
    static ref KOSCOM_IP: Ipv4Addr = get_koscom_ip().expect("Cannot find ip starts with 192.168");
}

fn get_koscom_ip() -> Option<Ipv4Addr> {
    let network_interfaces = list_afinet_netifas().unwrap();
    let mut res: Option<Ipv4Addr> = None;
    for (_, ip) in network_interfaces.iter() {
        let ip_string = &ip.to_string();
        if ip_string.len() > 7 && &ip.to_string()[..7] == "192.168" {
            match *ip {
                IpAddr::V4(ip) => res = Some(ip),
                _ => (),
            }
        }
    }
    res
}

pub struct MultiCast<'a> {
    group: Ipv4Addr,
    port: &'a str,
}

impl<'a> MultiCast<'a> {
    pub const fn new(group: Ipv4Addr, port: &'a str) -> Self {
        Self { group, port }
    }
    pub async fn joined_socket(&self) -> UdpSocket {
        match UdpSocket::bind(String::from("0.0.0.0:") + &self.port).await {
            Result::Ok(socket) => {
                socket
                    .join_multicast_v4(self.group, *KOSCOM_IP)
                    .expect("Cannot join");
                socket
            }
            Err(_s) => panic!("Cannot bind socket"),
        }
    }
}
