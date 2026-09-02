use crate::AppError;
use chrono::{NaiveDateTime, ParseError};
use csv_macro::CsvStruct;
use std::fmt;
use std::str::ParseBoolError;

#[derive(CsvStruct)]
pub struct Order<'sus> {
    order_id: &'sus str,
    customer_id: &'sus str,
    restaurant_id: &'sus str,
    driver_id: &'sus str,
    time_stamp: NaiveDateTime,
    // #[split_skip_to(7)]
    #[split_skip_amount(2)] // equivalent of the previous line for the current table
    day_of_week: &'sus str,
    is_weekend: bool,
    city: &'sus str,
    delivery_area: &'sus str,
}
