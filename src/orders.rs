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

impl<'a> Order<'a> {
    pub fn from_string(string: &'a str) -> Result<Self, AppError> {
        let split: Vec<&str> = string.split(',').collect();
        let timestamp: NaiveDateTime;

        match parse_datetime(split[4]) {
            Ok(val) => timestamp = val,
            Err(_) => return Err(AppError::FileParsingError(String::from(split[3]))),
        }

        let new = Self {
            order_id: split[0],
            customer_id: split[1],
            restaurant_id: split[2],
            driver_id: split[3],
            time_stamp: timestamp,
        };

        return Ok(new);
    }
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
