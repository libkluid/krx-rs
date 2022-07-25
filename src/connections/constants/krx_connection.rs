#![allow(dead_code)]
use local_ip_address::list_afinet_netifas;
use std::net::{IpAddr, Ipv4Addr};
use tokio::net::UdpSocket;
use lazy_static::lazy_static;

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
                _ => ()
            }
        }
    };
    res
}

pub mod stock {
    pub mod kospi {
        use super::super::Ipv4Addr;
        use super::super::MultiCast;

        const MCAST_MATCH: Ipv4Addr = Ipv4Addr::new(233, 37, 54, 117);
        const MCAST_ORDER_BOARD: Ipv4Addr = Ipv4Addr::new(233, 37, 54, 118);
        pub const MATCH_1: MultiCast = MultiCast::new(MCAST_MATCH, "19561");
        pub const MATCH_2: MultiCast = MultiCast::new(MCAST_MATCH, "19562");
        pub const MATCH_3: MultiCast = MultiCast::new(MCAST_MATCH, "19563");
        pub const MATCH_4: MultiCast = MultiCast::new(MCAST_MATCH, "19564");
        pub const MATCH_5: MultiCast = MultiCast::new(MCAST_MATCH, "19565");
        pub const ORDER_BOARD_1: MultiCast = MultiCast::new(MCAST_ORDER_BOARD, "19566");
        pub const ORDER_BOARD_2: MultiCast = MultiCast::new(MCAST_ORDER_BOARD, "19567");
    }
    pub mod kosdaq {
        use super::super::Ipv4Addr;
        use super::super::MultiCast;

        const MCAST_MATCH: Ipv4Addr = Ipv4Addr::new(233, 37, 54, 217);
        const MCAST_ORDER_BOARD: Ipv4Addr = Ipv4Addr::new(233, 37, 54, 218);
        pub const MATCH_1: MultiCast = MultiCast::new(MCAST_MATCH, "19761");
        pub const MATCH_2: MultiCast = MultiCast::new(MCAST_MATCH, "19762");
        pub const MATCH_3: MultiCast = MultiCast::new(MCAST_MATCH, "19763");
        pub const MATCH_4: MultiCast = MultiCast::new(MCAST_MATCH, "19764");
        pub const MATCH_5: MultiCast = MultiCast::new(MCAST_MATCH, "19765");
        pub const MATCH_6: MultiCast = MultiCast::new(MCAST_MATCH, "19766");
        pub const MATCH_7: MultiCast = MultiCast::new(MCAST_MATCH, "19767");
        pub const MATCH_8: MultiCast = MultiCast::new(MCAST_MATCH, "19768");
        pub const MATCH_9: MultiCast = MultiCast::new(MCAST_MATCH, "19769");
        pub const MATCH_10: MultiCast = MultiCast::new(MCAST_MATCH, "19770");
        pub const ORDER_BOARD_1: MultiCast = MultiCast::new(MCAST_ORDER_BOARD, "19771");
        pub const ORDER_BOARD_2: MultiCast = MultiCast::new(MCAST_ORDER_BOARD, "19772");
    }
}

pub mod future {
    pub mod kospi {
        use super::super::Ipv4Addr;
        use super::super::MultiCast;

        const MCAST_BIG: Ipv4Addr = Ipv4Addr::new(233, 37, 54, 171);
        const MCAST_MINI: Ipv4Addr = Ipv4Addr::new(233, 37, 54, 245);
        pub const BIG_ORDER_MATCH: MultiCast = MultiCast::new(MCAST_BIG, "15572"); // 오더보드, 체결
        pub const MINI_ORDER_MATCH: MultiCast = MultiCast::new(MCAST_MINI, "15352"); // 오더보드, 체결
    }
    pub mod kosdaq {
        use super::super::Ipv4Addr;
        use super::super::MultiCast;

        const MCAST: Ipv4Addr = Ipv4Addr::new(233, 37, 54, 172);
        pub const ORDER_MATCH: MultiCast = MultiCast::new(MCAST, "15582"); // 오더보드, 체결
    }
    pub mod sector {
        use super::super::Ipv4Addr;
        use super::super::MultiCast;

        const MCAST: Ipv4Addr = Ipv4Addr::new(233, 37, 54, 244);
        pub const ORDER_MATCH: MultiCast = MultiCast::new(MCAST, "15332"); // 오더보드, 체결
    }
    pub mod stock {
        use super::super::Ipv4Addr;
        use super::super::MultiCast;

        const MCAST: Ipv4Addr = Ipv4Addr::new(233, 37, 54, 174);
        pub const ORDER_MATCH: MultiCast = MultiCast::new(MCAST, "15592"); // 오더보드, 체결
    }
}

pub mod index {
    pub mod kospi {
        use super::super::Ipv4Addr;
        use super::super::MultiCast;

        const MCAST: Ipv4Addr = Ipv4Addr::new(233, 37, 54, 115);
        pub const INDEX: MultiCast = MultiCast::new(MCAST, "19523");
        pub const TRADER: MultiCast = MultiCast::new(MCAST, "19527"); // 거래원 
        pub const PROG_ORDER: MultiCast = MultiCast::new(MCAST, "19528"); // 프로그램 매매 호가 
        // 섹터, 레버리지 포함
    }
    pub mod kosdaq {
        use super::super::Ipv4Addr;
        use super::super::MultiCast;

        const MCAST: Ipv4Addr = Ipv4Addr::new(233, 37, 54, 155);
        pub const INDEX: MultiCast = MultiCast::new(MCAST, "19773");
    }
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
