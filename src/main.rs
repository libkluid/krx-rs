mod connections;
mod market_data;
mod server;

use std::io::BufWriter;
use connections::constants::krx_connection;
use server::udp::UdpServer;
use std::collections::HashSet;

use market_data::constants::data_classifier;
use market_data::filter_list::FilterList;
use market_data::structs::stock_order_board::StockOrderBoard;
use market_data::structs::stock_order_board_include_lp::StockOrderBoardIncludeLp;
use market_data::structs::kospi_big_future_order_board::KospiBigFutureOrderBoard;
use market_data::structs::kosdaq_future_order_board::KosdaqFutureOrderBoard;
use market_data::traits::market_snapshot::MktSnapshot;
use std::io::Write;

use std::error::Error;
use tokio::sync::mpsc;

#[allow(unreachable_code)]
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // let mut filter_list = FilterList::new(HashSet::new());

    let (tx, mut rx): (mpsc::Sender<[u8; 1024]>, mpsc::Receiver<[u8; 1024]>) = mpsc::channel(1024);

    let socket1 = krx_connection::stock::kospi::ORDER_BOARD_1
        .joined_socket()
        .await;
    let socket2 = krx_connection::stock::kospi::ORDER_BOARD_2
        .joined_socket()
        .await;
    let socket3 = krx_connection::future::kosdaq::ORDER_MATCH.joined_socket().await;

    let server1 = UdpServer::new(String::from("1111111"), socket1, [0; 1024]);
    let server2 = UdpServer::new(String::from("2222222"), socket2, [0; 1024]);
    let server3 = UdpServer::new(String::from("3333333"), socket3, [0; 1024]);

    let _t1 = tokio::task::spawn(server1.run(tx.clone()));
    let _t2 = tokio::task::spawn(server2.run(tx.clone()));
    let _t3 = tokio::task::spawn(server3.run(tx.clone()));

    let mut stdout = std::io::stdout();
    let mut lock = stdout.lock();
    
    loop {
        match rx.recv().await {
            Some(data) => {
                match &data[..5] {
                    data_classifier::order_board::KOSPI_INCLUDE_LP => {
                        let order = StockOrderBoardIncludeLp::parse(&data);
                        if order.ticker == "229200" {
                            writeln!(
                                lock,
                                "{:0<50} {: <20} {: <10} {: <10} {: <10} {: <10} {: <0}",
                                order.ts,
                                order.ticker,
                                order.bidq1 - order.lp_bidq1,
                                order.bid1,
                                order.ask1,
                                order.askq1 - order.lp_askq1,
                                order.toq
                            )?;                          
                        }
                    },
                    data_classifier::order_board::KOSDAQ150_FUTURE => {
                        let order = KosdaqFutureOrderBoard::parse(&data);
                        if order.ticker == "106S9000" {
                            writeln!(
                                lock,
                                "-----> {:0<50} {: <20} {: <10} {: <10} {: <10} {: <0}",
                                order.ts,
                                order.ticker,
                                order.bidq1,
                                order.bid1,
                                order.ask1,
                                order.askq1,
                            )?;                          
                        }
                    }
                    _ => {}
                }
            }
            None => (),
        }
    }

    Ok(())
}
