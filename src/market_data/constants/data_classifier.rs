#![allow(dead_code)]
use crate::market_data::data_handler::DataHandler;
use crate::market_data::traits::parser;

pub mod classifier {
    pub const MATCH_DATA: &[u8] = b"A3";
    pub const ORDER_BOARD: &[u8] = b"B6";
    pub const ORDER_BOARD_INCLUDE_LP: &[u8] = b"B7";
    pub const KOSPI_PROG: &[u8] = b"C3";
    pub const KOSPI_200_INDEX: &[u8] = b"D2";
    pub const KRX_SECTOR_INDEX: &[u8] = b"E0";
    pub const KOSPI_SECTOR_INDEX: &[u8] = b"N5";
    pub const KOSPI_TRADER: &[u8] = b"B9";
    pub const KOSDAQ_INDEX: &[u8] = b"E4"; // 코스닥_지수(소문자인지 확인), 길이 = INDEX_LEN
}

pub mod nav {
    pub const KOSPI: &[u8] = b"BV011";

    pub const LEN: usize = 70; // ETF_NAV_(주기단축)
}

pub mod order_match {
    pub const KOSPI_STOCK: &[u8] = b"A3011"; // 체결 코스피 주식
    pub const KOSDAQ_STOCK: &[u8] = b"A3012"; // 체결 코스닥 주식
    pub const KOSPI_ELW: &[u8] = b"A3021"; // 체결 코스피 ELW
    pub const KOSPI_FUTURE: &[u8] = b"A3014"; // 체결 코스피 선물
    pub const STOCK_FUTURE: &[u8] = b"A3015"; // 주식선물 체결
    pub const KOSPI_MINI_FUTURE: &[u8] = b"A3124"; // 미니 코스피200 선물 체결
    pub const SECTOR_FUTURE: &[u8] = b"A3104"; // 섹터지수선물 체결
    pub const KOSDAQ150_FUTURE: &[u8] = b"A3024"; // 코스닥150선물 체결

    pub const KOSPI_STOCK_LEN: usize = 160; // 코스피_체결
    pub const KOSDAQ_STOCK_LEN: usize = 160; // 코스닥_체결
    pub const KOSDAQ_FUTURE_LEN: usize = 125; // 코스닥150선물_체결
    pub const KOSPI_BIG_FUTURE_LEN: usize = 117; // K200선물_체결
    pub const KOSPI_MINI_FUTURE_LEN: usize = 117; // 미니K200선물_체결
    pub const STOCK_FUTURE_LEN: usize = 133; // 주식선물_체결
    pub const SECTOR_FUTURE_LEN: usize = 170; // 섹터지수선물_체결
}

pub mod order_board {
    pub const KOSPI_ELW_INCLUDE_LP: &[u8] = b"B7021"; // 호가잔량_LP호가 포함 ELW
    pub const KOSPI_INCLUDE_LP: &[u8] = b"B7011"; // 호가잔량_LP호가 포함 코스피 주식
    pub const KOSPI_EXCLUDE_LP: &[u8] = b"B6011"; // 호가잔량_LP호가 제외 코스피 주식
    pub const KOSDAQ_EXCLUDE_LP: &[u8] = b"B6012"; // 호가잔량_LP호가 제외 코스닥 주식

    pub const KOSPI200_FUTURE: &[u8] = b"B6014"; // 코스피200 선물 호가
    pub const STOCK_FUTURE: &[u8] = b"B6015"; // 주식선물 호가
    pub const KOSPI_MINI: &[u8] = b"B6124"; // 미니 코스피200 선물 호가
    pub const SECTOR_FUTURE: &[u8] = b"B6104"; // 섹터지수선물 호가
    pub const KOSDAQ150_FUTURE: &[u8] = b"B6024"; // 코스닥150선물 호가

    pub const KOSPI_STOCK_EXCLUDE_LP_LEN: usize = 560; // 코스피_호가잔량_LP호가_제외
    pub const KOSPI_STOCK_INCLUDE_LP_LEN: usize = 800; // 코스피_호가잔량_LP호가_포함
    pub const KOSDAQ_STOCK_LEN: usize = 560; // 코스닥_호가잔량_LP_호가_제외
    pub const KOSPI_BIG_FUTURE_LEN: usize = 220; // K200선물_우선호가
    pub const KOSPI_MINI_FUTURE_LEN: usize = 220; // 미니K200선물_우선호가
    pub const KOSDAQ_FUTURE_LEN: usize = 231; // 코스닥150선물_우선호가
    pub const STOCK_FUTURE_LEN: usize = 448; // 주식선물_우선호가
    pub const SECTOR_FUTURE_LEN: usize = 320; // 섹터지수선물_우선호가
}

pub mod prog_order {
    pub const KOSPI: &[u8] = b"C3011"; // 프로그램 매매호가 코스피
    pub const KOSDAQ: &[u8] = b"C3012"; // 프로그램 매매호가 코스닥

    pub const LEN: usize = 460;
}

pub mod index {
    pub const KOSPI200: &[u8] = b"D2011"; // 코스피 200 지수
    pub const KRX_SECTOR: &[u8] = b"E0011"; // KRX 섹터지수 ??
    pub const KOSPI_SECTOR: &[u8] = b"N5011"; // 코스피 섹터지수
    pub const KOSDAQ: &[u8] = b"E4012"; // 코스닥 지수

    pub const LEN: usize = 50;
}

pub mod member {
    pub const KOSPI_STOCK_MATCH: &[u8] = b"B9011"; // 코스피 거래원
    pub const KOSDAQ_STOCK_MATCH: &[u8] = b"B9012"; // 코스닥 거래원
    pub const KOSPI_ELW_MATCH: &[u8] = b"B9021"; // 코스피 ELW 거래원

    pub const LEN: usize = 380;
}

pub fn subscribe_data_classifiers_map(classifier: &[u8]) -> DataHandler {
    match classifier {
        nav::KOSPI => DataHandler::NAV_KOSPI,
        order_match::KOSPI_STOCK => DataHandler::ORDER_MATCH_KOSPI_STOCK, // 체결 코스피 주식
        order_match::KOSDAQ_STOCK => DataHandler::ORDER_MATCH_KOSDAQ_STOCK, // 체결 코스닥 주식

        // order_match::KOSPI_ELW => DataHandler::ORDER_MATCH_KOSPI_ELW, // 체결 코스피 ELW
        // order_match::KOSDAQ_ELW => DataHandler::ORDER_MATCH_KOSDAQ_ELW, // 체결 코스닥 ELW

        order_match::KOSPI_FUTURE => DataHandler::ORDER_MATCH_KOSPI_FUTURE, // 체결 코스피 선물
        order_match::STOCK_FUTURE => DataHandler::ORDER_MATCH_STOCK_FUTURE, // 주식선물 체결
        order_match::KOSPI_MINI_FUTURE => DataHandler::ORDER_MATCH_KOSPI_MINI_FUTURE, // 미니 코스피200 선물 체결
        order_match::SECTOR_FUTURE => DataHandler::ORDER_MATCH_SECTOR_FUTURE, // 섹터지수선물 체결
        order_match::KOSDAQ150_FUTURE => DataHandler::ORDER_MATCH_KOSDAQ150_FUTURE, // 코스닥150선물 체결

        // order_board::KOSPI_ELW_INCLUDE_LP => DataHandler::ORDER_BOARD_KOSPI_ELW_INCLUDE_LP, // 호가잔량_LP호가 포함 ELW
        order_board::KOSPI_INCLUDE_LP => DataHandler::ORDER_BOARD_KOSPI_INCLUDE_LP, // 호가잔량_LP호가 포함 코스피 주식
        order_board::KOSPI_EXCLUDE_LP => DataHandler::ORDER_BOARD_KOSPI_EXCLUDE_LP, // 호가잔량_LP호가 제외 코스피 주식
        order_board::KOSDAQ_EXCLUDE_LP => DataHandler::ORDER_BOARD_KOSDAQ_EXCLUDE_LP, // 호가잔량_LP호가 제외 코스닥 주식

        order_board::KOSPI200_FUTURE => DataHandler::ORDER_BOARD_KOSPI200_FUTURE, // 코스피200 선물 호가
        order_board::STOCK_FUTURE => DataHandler::ORDER_BOARD_STOCK_FUTURE,       // 주식선물 호가
        order_board::KOSPI_MINI => DataHandler::ORDER_BOARD_KOSPI_MINI, // 미니 코스피200 선물 호가
        order_board::SECTOR_FUTURE => DataHandler::ORDER_BOARD_SECTORE_FUTURE, // 섹터지수선물 호가
        order_board::KOSDAQ150_FUTURE => DataHandler::ORDER_BOARD_KOSDAQ150_FUTURE, // 코스닥150선물 호가

        // prog_order::KOSPI => DataHandler::PROG_ORDER_KOSPI, // 프로그램 매매호가 코스피
        // prog_order::KOSDAQ => DataHandler::PROG_ORDER_KOSDAQ, // 프로그램 매매호가 코스닥

        // index::KOSPI200 => DataHandler::INDEX_KOSPI200, // 코스피 200 지수
        // index::KRX_SECTOR => DataHandler::INDEX_KRX_SECTOR, // KRX 섹터지수
        // index::KOSPI_SECTOR => DataHandler::INDEX_KOSPI_SECTOR, // 코스피 섹터지수
        // index::KOSDAQ => DataHandler::INDEX_KOSDAQ, // 코스닥 지수
        member::KOSPI_STOCK_MATCH => DataHandler::MEMBER_KOSPI_STOCK_MATCH, // 코스피 거래원
        member::KOSDAQ_STOCK_MATCH => DataHandler::MEMBER_KOSDAQ_STOCK_MATCH, // 코스닥 거래원
        // member::KOSPI_ELW_MATCH => DataHandler::MEMBER_KOSPI_ELW_MATCH, // 코스피 ELW 거래원
        // member::KOSDAQ_ELW_MATCH => DataHandler::MEMBER_KOSDAQ_ELW_MATCH, // 코스닥 ELW 거래원
        _ => DataHandler::UNKNOWN,
    }
}

// pub fn parse_packet(classifier: &[u8]) -> parser::ParsePacket {
    // match classifier {
        // nav::KOSPI => DataHandler::NAV_KOSPI,
        // nav::KOSDAQ => DataHandler::NAV_KOSDAQ,
        // order_match::KOSPI_STOCK => DataHandler::ORDER_MATCH_KOSPI_STOCK, // 체결 코스피 주식
        // order_match::KOSDAQ_STOCK => DataHandler::ORDER_MATCH_KOSDAQ_STOCK, // 체결 코스닥 주식

        // order_match::KOSPI_ELW => DataHandler::ORDER_MATCH_KOSPI_ELW, // 체결 코스피 ELW
        // order_match::KOSDAQ_ELW => DataHandler::ORDER_MATCH_KOSDAQ_ELW, // 체결 코스닥 ELW

        // order_match::KOSPI_FUTURE => DataHandler::ORDER_MATCH_KOSPI_FUTURE, // 체결 코스피 선물
        // order_match::STOCK_FUTURE => DataHandler::ORDER_MATCH_STOCK_FUTURE, // 주식선물 체결
        // order_match::KOSPI_MINI_FUTURE => DataHandler::ORDER_MATCH_KOSPI_MINI_FUTURE, // 미니 코스피200 선물 체결
        // order_match::SECTOR_FUTURE => DataHandler::ORDER_MATCH_SECTOR_FUTURE, // 섹터지수선물 체결
        // order_match::KOSDAQ150_FUTURE => DataHandler::ORDER_MATCH_KOSDAQ150_FUTURE, // 코스닥150선물 체결

        // order_board::KOSPI_ELW_INCLUDE_LP => DataHandler::ORDER_BOARD_KOSPI_ELW_INCLUDE_LP, // 호가잔량_LP호가 포함 ELW
        // order_board::KOSPI_INCLUDE_LP => DataHandler::ORDER_BOARD_KOSPI_INCLUDE_LP, // 호가잔량_LP호가 포함 코스피 주식
        // ORDER_BOARD_KOSPI_EXCLUDE_LP => DataHandler::ORDER_BOARD_KOSPI_EXCLUDE_LP, // 호가잔량_LP호가 제외 코스피 주식
        // order_board::KOSDAQ_EXCLUDE_LP => DataHandler::ORDER_BOARD_KOSDAQ_EXCLUDE_LP, // 호가잔량_LP호가 제외 코스닥 주식

        // order_board::KOSPI200_FUTURE => DataHandler::ORDER_BOARD_KOSPI200_FUTURE, // 코스피200 선물 호가
        // order_board::STOCK_FUTURE => DataHandler::ORDER_BOARD_STOCK_FUTURE,       // 주식선물 호가
        // order_board::KOSPI_MINI => DataHandler::ORDER_BOARD_KOSPI_MINI, // 미니 코스피200 선물 호가
        // order_board::SECTORE_FUTURE => DataHandler::ORDER_BOARD_SECTORE_FUTURE, // 섹터지수선물 호가
        // order_board::KOSDAQ150_FUTURE => DataHandler::ORDER_BOARD_KOSDAQ150_FUTURE, // 코스닥150선물 호가

        // prog_order::KOSPI => DataHandler::PROG_ORDER_KOSPI, // 프로그램 매매호가 코스피
        // prog_order::KOSDAQ => DataHandler::PROG_ORDER_KOSDAQ, // 프로그램 매매호가 코스닥

        // index::KOSPI200 => DataHandler::INDEX_KOSPI200, // 코스피 200 지수
        // index::KRX_SECTOR => DataHandler::INDEX_KRX_SECTOR, // KRX 섹터지수
        // index::KOSPI_SECTOR => DataHandler::INDEX_KOSPI_SECTOR, // 코스피 섹터지수
        // index::KOSDAQ => DataHandler::INDEX_KOSDAQ, // 코스닥 지수
        // member::KOSPI_STOCK_MATCH => DataHandler::MEMBER_KOSPI_STOCK_MATCH, // 코스피 거래원
        // member::KOSDAQ_STOCK_MATCH => DataHandler::MEMBER_KOSDAQ_STOCK_MATCH, // 코스닥 거래원
        // member::KOSPI_ELW_MATCH => DataHandler::MEMBER_KOSPI_ELW_MATCH, // 코스피 ELW 거래원
        // member::KOSDAQ_ELW_MATCH => DataHandler::MEMBER_KOSDAQ_ELW_MATCH, // 코스닥 ELW 거래원
        // _ => DataHandler::UNKNOWN,
    // }
// }
