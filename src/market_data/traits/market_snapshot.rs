use chrono::prelude::{DateTime, Local};

pub trait MktSnapshot {
    fn ts(&self) -> DateTime<Local>;
    fn ticker(&self) -> &str;
    fn ask(&self) -> f64;
    fn bid(&self) -> f64;
    fn askq(&self) -> u32;
    fn bidq(&self) -> u32;
}
