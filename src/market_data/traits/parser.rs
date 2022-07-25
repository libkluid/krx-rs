use atoi::FromRadix10;
use lexical;

pub trait NumberParser {
    fn parse_price(packet: &[u8], start: usize, end: usize) -> f64 {
        f64::from_radix_10(&packet[start..end]).0
    }

    fn parse_qty(packet: &[u8], start: usize, end: usize) -> u32 {
        u32::from_radix_10(&packet[start..end]).0
    }

    unsafe fn parse_pts(packet: &[u8], start: usize, end: usize) -> f64 {
        let joiner: &str = ".";
        let integer = std::str::from_utf8_unchecked(&packet[start..end-2]);
        let floating = std::str::from_utf8_unchecked(&packet[end-2..end]);
        let num: &str = &[integer, floating].join(joiner);
        lexical::parse::<f64, _>(num).expect("Invalid float number")
    }
}

pub trait StockTickerParser {
    fn convert_isin_short_code_priority(priority: &str) -> &str {
        match priority {
            "0" => "0",
            "1" => "5",
            "2" => "7",
            "3" => "9",
            "K" => "K",
            _ => todo!()
        }
    }

    unsafe fn parse_ticker(packet: &[u8], start: usize, end: usize) -> String {
        let code = std::str::from_utf8_unchecked(&packet[start..end-1]);
        let priority = Self::convert_isin_short_code_priority(std::str::from_utf8_unchecked(&packet[end-1..end]));
        [code, priority].join("")
    }
}

pub trait FutureTickerParser {
    unsafe fn parse_ticker(packet: &[u8], start: usize, end: usize) -> String {
        String::from(std::str::from_utf8_unchecked(&packet[start..end]))
    }
}
