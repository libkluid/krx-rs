#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_must_use)]

use std::io;
use tokio::net::UdpSocket;
use tokio::sync::mpsc;
use crate::market_data::constants::data_classifier::subscribe_data_classifiers_map;
use crate::market_data::data_handler::DataHandler;

pub struct UdpServer {
    name: String,
    socket: UdpSocket,
    buf: [u8; 1024],
}

impl UdpServer {
    pub fn new(name: String, socket: UdpSocket, buf: [u8; 1024]) -> Self {
        Self { name, socket, buf }
    }

    pub fn get_data_class(packet: &[u8]) -> &[u8] {
        &packet[..5]
    }
    
    pub fn valid_data_class(data_class: &[u8]) -> bool {
        subscribe_data_classifiers_map(data_class) != DataHandler::UNKNOWN
    }

    pub async fn run(self, tx: mpsc::Sender<[u8;1024]>) -> Result<(), io::Error> {
        let UdpServer {
            name,
            socket,
            mut buf
        } = self;
        loop {
            // let addr = socket.local_addr().unwrap().ip();
            let (len, addr) = socket.recv_from(&mut buf).await?;
            let data_class = UdpServer::get_data_class(&buf);
            if UdpServer::valid_data_class(data_class) {
                tx.send(buf).await;
            }
        }
    }
}
