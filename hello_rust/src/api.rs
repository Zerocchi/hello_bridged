use anyhow::{anyhow, Result};

use flutter_rust_bridge::ZeroCopyBuffer;

pub fn add_counter(number: i32) -> Result<i32> {
    Ok(number + 1)
}