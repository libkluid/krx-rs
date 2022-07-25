#[derive(PartialEq)]
pub enum DataHandler {
    NAV_KOSPI,
    NAV_KOSDAQ,

    ORDER_MATCH_KOSPI_STOCK, // 체결 코스피 주식
    ORDER_MATCH_KOSDAQ_STOCK, // 체결 코스닥 주식
    ORDER_MATCH_KOSPI_ELW, // 체결 코스피 ELW
    ORDER_MATCH_KOSDAQ_ELW, // 체결 코스닥 ELW
    ORDER_MATCH_KOSPI_FUTURE, // 체결 코스피 선물
    ORDER_MATCH_STOCK_FUTURE, // 주식선물 체결
    ORDER_MATCH_KOSPI_MINI_FUTURE, // 미니 코스피200 선물 체결
    ORDER_MATCH_SECTOR_FUTURE, // 섹터지수선물 체결
    ORDER_MATCH_KOSDAQ150_FUTURE, // 코스닥150선물 체결

    ORDER_BOARD_KOSPI_ELW_INCLUDE_LP, // 호가잔량_LP호가 포함 ELW
    ORDER_BOARD_KOSPI_INCLUDE_LP, // 호가잔량_LP호가 포함 코스피 주식
    ORDER_BOARD_KOSPI_EXCLUDE_LP, // 호가잔량_LP호가 제외 코스피 주식
    ORDER_BOARD_KOSDAQ_EXCLUDE_LP, // 호가잔량_LP호가 제외 코스닥 주식

    ORDER_BOARD_KOSPI200_FUTURE, // 코스피200 선물 호가
    ORDER_BOARD_STOCK_FUTURE, // 주식선물 호가
    ORDER_BOARD_KOSPI_MINI, // 미니 코스피200 선물 호가
    ORDER_BOARD_SECTORE_FUTURE, // 섹터지수선물 호가
    ORDER_BOARD_KOSDAQ150_FUTURE, // 코스닥150선물 호가

    PROG_ORDER_KOSPI, // 프로그램 매매호가 코스피
    PROG_ORDER_KOSDAQ, // 프로그램 매매호가 코스닥

    INDEX_KOSPI200, // 코스피 200 지수
    INDEX_KRX_SECTOR, // KRX 섹터지수
    INDEX_KOSPI_SECTOR, // 코스피 섹터지수
    INDEX_KOSDAQ, // 코스닥 지수

    MEMBER_KOSPI_STOCK_MATCH, // 코스피 거래원
    MEMBER_KOSDAQ_STOCK_MATCH, // 코스닥 거래원
    MEMBER_KOSPI_ELW_MATCH, // 코스피 ELW 거래원
    MEMBER_KOSDAQ_ELW_MATCH, // 코스닥 ELW 거래원
    UNKNOWN,
}