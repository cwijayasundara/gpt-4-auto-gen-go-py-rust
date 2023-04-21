use trademicroservice::db::Trade;
use chrono::NaiveDateTime;

#[test]
fn test_trade_creation() {
    let trade = Trade {
        id: 1,
        value: 100.0,
        date: NaiveDateTime::from_timestamp(0, 0),
        trader: String::from("Trader1"),
    };

    assert_eq!(trade.id, 1);
    assert_eq!(trade.value, 100.0);
    assert_eq!(trade.date, NaiveDateTime::from_timestamp(0, 0));
    assert_eq!(trade.trader, String::from("Trader1"));
}
