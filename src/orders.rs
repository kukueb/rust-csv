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
