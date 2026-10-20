# rust-csv

A lightweight, minimal-dependency CSV parser in pure Rust powered by a custom procedural derive macro (`CsvStruct`). It allows deserializing CSV records directly into Rust structs with zero-copy string borrowing, flexible column skipping/indexing, and easily extensible type parsing.

## Features

- **Minimal Dependencies**: Relies exclusively on standard library I/O, standard procedural macro crates (`syn`, `quote`), and `chrono` for date/time support. No bulky CSV engines or heavy runtime serialization frameworks.
- **Custom Procedural Macro (`#[derive(CsvStruct)]`)**: Automatically derives record parsing (`from_string`) and formatted `Display` implementations.
- **Zero-Copy Slicing**: Binds string fields as borrowed slices (`&'a str`) directly from row buffers without unnecessary allocations.
- **Flexible Column Mapping & Skipping**:
  - `#[split_index(n)]` — maps the field to an absolute column index.
  - `#[split_skip_to(n)]` — advances the parser directly to column `n`.
  - `#[split_skip_amount(n)]` — skips `n` columns relative to the current position.
- **Extensible Type Support**: Simple architecture for adding new data types and custom parsing rules via type classification (`classify`) and code generation (`gen_construction_for_type`) in `csv-macro`. Currently supports `&str`, `bool` (including `0`/`1`), and `NaiveDateTime`.

## Usage Example

Define your record struct and annotate fields as needed:

```rust
use chrono::NaiveDateTime;
use csv_macro::CsvStruct;

#[derive(CsvStruct)]
pub struct Order<'a> {
    order_id: &'a str,
    customer_id: &'a str,
    restaurant_id: &'a str,
    driver_id: &'a str,
    time_stamp: NaiveDateTime,
    #[split_skip_amount(2)] // skip next 2 columns in the CSV
    day_of_week: &'a str,
    is_weekend: bool,
    city: &'a str,
    delivery_area: &'a str,
}
```

Parse records from raw CSV strings:

```rust
let line = "ORD_000001,CUS_00001,RES_0008,DRV_0075,2025-10-05 15:54:00,2025-10-05,15,Sunday,1,City_C,City_C_Area_3";

let order = Order::from_string(line)?;
println!("{}", order);
```

## Running

```bash
cargo run -- data/food_delivery_orders_dataset.csv
```
