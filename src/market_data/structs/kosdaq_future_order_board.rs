#![allow(dead_code)]

use crate::market_data::traits::market_snapshot::MktSnapshot;
use crate::market_data::traits::parser::FutureTickerParser;
use crate::market_data::traits::parser::NumberParser;
use chrono::prelude::{DateTime, Local};

#[derive(Debug)]
pub struct KosdaqFutureOrderBoard {
    pub ts: DateTime<Local>,
    pub ticker: String,
    pub ask1: f64,
    pub ask2: f64,
    pub ask3: f64,
    pub ask4: f64,
    pub ask5: f64,
    pub bid1: f64,
    pub bid2: f64,
    pub bid3: f64,
    pub bid4: f64,
    pub bid5: f64,
    pub askq1: u32,
    pub askq2: u32,
    pub askq3: u32,
    pub askq4: u32,
    pub askq5: u32,
    pub bidq1: u32,
    pub bidq2: u32,
    pub bidq3: u32,
    pub bidq4: u32,
    pub bidq5: u32,
}

impl KosdaqFutureOrderBoard {
    pub fn new(
        ts: DateTime<Local>,
        ticker: String,
        ask1: f64,
        ask2: f64,
        ask3: f64,
        ask4: f64,
        ask5: f64,
        bid1: f64,
        bid2: f64,
        bid3: f64,
        bid4: f64,
        bid5: f64,
        askq1: u32,
        askq2: u32,
        askq3: u32,
        askq4: u32,
        askq5: u32,
        bidq1: u32,
        bidq2: u32,
        bidq3: u32,
        bidq4: u32,
        bidq5: u32,
    ) -> Self {
        Self {
            ts,
            ticker,
            ask1,
            ask2,
            ask3,
            ask4,
            ask5,
            bid1,
            bid2,
            bid3,
            bid4,
            bid5,
            askq1,
            askq2,
            askq3,
            askq4,
            askq5,
            bidq1,
            bidq2,
            bidq3,
            bidq4,
            bidq5,
        }
    }

    pub fn parse(packet: &[u8]) -> Self {
        Self {
            ts: Local::now(),
            ticker: unsafe { Self::parse_ticker(packet, 8, 16) },
            bid1: unsafe { Self::parse_pts(packet, 30, 36) },
            bidq1: Self::parse_qty(packet, 36, 42),
            bid2: unsafe { Self::parse_pts(packet, 43, 49) },
            bidq2: Self::parse_qty(packet, 49, 55),
            bid3: unsafe { Self::parse_pts(packet, 56, 62) },
            bidq3: Self::parse_qty(packet, 62, 68),
            bid4: unsafe { Self::parse_pts(packet, 69, 75) },
            bidq4: Self::parse_qty(packet, 75, 81),
            bid5: unsafe { Self::parse_pts(packet, 82, 88) },
            bidq5: Self::parse_qty(packet, 88, 94),
            ask1: unsafe { Self::parse_pts(packet, 101, 107) },
            askq1: Self::parse_qty(packet, 107, 113),
            ask2: unsafe { Self::parse_pts(packet, 114, 120) },
            askq2: Self::parse_qty(packet, 120, 126),
            ask3: unsafe { Self::parse_pts(packet, 127, 133) },
            askq3: Self::parse_qty(packet, 133, 139),
            ask4: unsafe { Self::parse_pts(packet, 140, 146) },
            askq4: Self::parse_qty(packet, 146, 152),
            ask5: unsafe { Self::parse_pts(packet, 153, 159) },
            askq5: Self::parse_qty(packet, 159, 165),
        }
    }
}

impl FutureTickerParser for KosdaqFutureOrderBoard {}
impl NumberParser for KosdaqFutureOrderBoard {}

impl MktSnapshot for KosdaqFutureOrderBoard {
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
