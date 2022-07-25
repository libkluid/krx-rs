#![allow(dead_code)]

use crate::market_data::traits::market_snapshot::MktSnapshot;
use crate::market_data::traits::parser::NumberParser;
use crate::market_data::traits::parser::StockTickerParser;
use chrono::prelude::{DateTime, Local};

#[derive(Debug)]
pub struct StockOrderBoard {
    pub ts: DateTime<Local>,
    pub ticker: String,
    pub toq: u32,
    pub ask1: f64,
    pub ask2: f64,
    pub ask3: f64,
    pub ask4: f64,
    pub ask5: f64,
    pub ask6: f64,
    pub ask7: f64,
    pub ask8: f64,
    pub ask9: f64,
    pub ask10: f64,
    pub bid1: f64,
    pub bid2: f64,
    pub bid3: f64,
    pub bid4: f64,
    pub bid5: f64,
    pub bid6: f64,
    pub bid7: f64,
    pub bid8: f64,
    pub bid9: f64,
    pub bid10: f64,
    pub askq1: u32,
    pub askq2: u32,
    pub askq3: u32,
    pub askq4: u32,
    pub askq5: u32,
    pub askq6: u32,
    pub askq7: u32,
    pub askq8: u32,
    pub askq9: u32,
    pub askq10: u32,
    pub bidq1: u32,
    pub bidq2: u32,
    pub bidq3: u32,
    pub bidq4: u32,
    pub bidq5: u32,
    pub bidq6: u32,
    pub bidq7: u32,
    pub bidq8: u32,
    pub bidq9: u32,
    pub bidq10: u32,
}

impl StockOrderBoard {
    pub fn new(
        ts: DateTime<Local>,
        ticker: String,
        toq: u32,
        ask1: f64,
        ask2: f64,
        ask3: f64,
        ask4: f64,
        ask5: f64,
        ask6: f64,
        ask7: f64,
        ask8: f64,
        ask9: f64,
        ask10: f64,
        bid1: f64,
        bid2: f64,
        bid3: f64,
        bid4: f64,
        bid5: f64,
        bid6: f64,
        bid7: f64,
        bid8: f64,
        bid9: f64,
        bid10: f64,
        askq1: u32,
        askq2: u32,
        askq3: u32,
        askq4: u32,
        askq5: u32,
        askq6: u32,
        askq7: u32,
        askq8: u32,
        askq9: u32,
        askq10: u32,
        bidq1: u32,
        bidq2: u32,
        bidq3: u32,
        bidq4: u32,
        bidq5: u32,
        bidq6: u32,
        bidq7: u32,
        bidq8: u32,
        bidq9: u32,
        bidq10: u32,
    ) -> Self {
        Self {
            ts,
            ticker,
            toq,
            ask1,
            ask2,
            ask3,
            ask4,
            ask5,
            ask6,
            ask7,
            ask8,
            ask9,
            ask10,
            bid1,
            bid2,
            bid3,
            bid4,
            bid5,
            bid6,
            bid7,
            bid8,
            bid9,
            bid10,
            askq1,
            askq2,
            askq3,
            askq4,
            askq5,
            askq6,
            askq7,
            askq8,
            askq9,
            askq10,
            bidq1,
            bidq2,
            bidq3,
            bidq4,
            bidq5,
            bidq6,
            bidq7,
            bidq8,
            bidq9,
            bidq10,
        }
    }

    pub fn parse(packet: &[u8]) -> Self {
        Self {
            ts: Local::now(),
            // isin: unsafe {
            //     String::from(std::str::from_utf8_unchecked(&packet[5..17]))
            // },
            ticker: unsafe { Self::parse_ticker(packet, 8, 14) },
            toq: Self::parse_qty(packet, 22, 34),
            ask1: Self::parse_price(packet, 34, 43),
            bid1: Self::parse_price(packet, 43, 52),
            askq1: Self::parse_qty(packet, 52, 64),
            bidq1: Self::parse_qty(packet, 64, 76),
            ask2: Self::parse_price(packet, 76, 85),
            bid2: Self::parse_price(packet, 85, 94),
            askq2: Self::parse_qty(packet, 94, 106),
            bidq2: Self::parse_qty(packet, 106, 118),
            ask3: Self::parse_price(packet, 118, 127),
            bid3: Self::parse_price(packet, 127, 136),
            askq3: Self::parse_qty(packet, 136, 148),
            bidq3: Self::parse_qty(packet, 148, 160),
            ask4: Self::parse_price(packet, 160, 169),
            bid4: Self::parse_price(packet, 169, 178),
            askq4: Self::parse_qty(packet, 178, 190),
            bidq4: Self::parse_qty(packet, 190, 202),
            ask5: Self::parse_price(packet, 202, 211),
            bid5: Self::parse_price(packet, 211, 220),
            askq5: Self::parse_qty(packet, 220, 232),
            bidq5: Self::parse_qty(packet, 232, 244),
            ask6: Self::parse_price(packet, 244, 253),
            bid6: Self::parse_price(packet, 253, 262),
            askq6: Self::parse_qty(packet, 262, 274),
            bidq6: Self::parse_qty(packet, 274, 286),
            ask7: Self::parse_price(packet, 286, 295),
            bid7: Self::parse_price(packet, 295, 304),
            askq7: Self::parse_qty(packet, 304, 316),
            bidq7: Self::parse_qty(packet, 316, 328),
            ask8: Self::parse_price(packet, 328, 337),
            bid8: Self::parse_price(packet, 337, 346),
            askq8: Self::parse_qty(packet, 346, 358),
            bidq8: Self::parse_qty(packet, 358, 370),
            ask9: Self::parse_price(packet, 370, 379),
            bid9: Self::parse_price(packet, 379, 388),
            askq9: Self::parse_qty(packet, 388, 400),
            bidq9: Self::parse_qty(packet, 400, 412),
            ask10: Self::parse_price(packet, 412, 421),
            bid10: Self::parse_price(packet, 421, 430),
            askq10: Self::parse_qty(packet, 430, 442),
            bidq10: Self::parse_qty(packet, 442, 454),
        }
    }
}

impl StockTickerParser for StockOrderBoard {}
impl NumberParser for StockOrderBoard {}

impl MktSnapshot for StockOrderBoard {
    fn ts(&self) -> DateTime<Local> {
        self.ts
    }
    fn ticker(&self) -> &str {
        self.ticker.as_ref()
    }
    fn ask(&self) -> f64 {
        self.ask1
    }
    fn bid(&self) -> f64 {
        self.bid1
    }
    fn askq(&self) -> u32 {
        self.askq1
    }
    fn bidq(&self) -> u32 {
        self.bidq1
    }
}
