use crate::AppError;
use chrono::{NaiveDateTime, ParseError};
use csv_macro::CsvStruct;
use std::fmt;

#[derive(CsvStruct)]
pub struct Order<'sus> {
    order_id: &'sus str,
    customer_id: &'sus str,
    restaurant_id: &'sus str,
    driver_id: &'sus str,
    time_stamp: NaiveDateTime,
}

impl fmt::Display for Order<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let timestamp = self.time_stamp.format("%Y-%m-%d %H:%M:%S");

        return write!(
            f,
            "
order_id = {},
customer_id = {},
restaurant_id = {},
driver_id = {},
time_stamp={}
",
            self.order_id, self.customer_id, self.restaurant_id, self.driver_id, timestamp
        );
    }
}
fn parse_datetime(string: &str) -> Result<NaiveDateTime, ParseError> {
    return NaiveDateTime::parse_from_str(string, "%Y-%m-%d %H:%M:%S");
}
