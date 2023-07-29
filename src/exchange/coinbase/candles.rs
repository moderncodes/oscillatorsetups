use crate::utils::{data_from_json, data_to_json, CustomError,get_folder_path};
use super::{fetch::products, models::{ApiParams,Klines,},};
use std::error::Error;
use serde_json::from_str;

/// Retrieves candle data from a remote source and stores it into a local file.
#[allow(dead_code)]
pub fn remote_to_file(
    base_url    : &str,
    granularity : u32,
    limit       : u16,
    product_id  : &str
) -> Result<Vec<Klines>, Box<dyn Error>> {
    let candle_data = candles("api", base_url, granularity, limit, product_id)?;
    let json = serde_json::to_string(&candle_data).unwrap_or_else(|_| panic!("Failed to serialize data"));
    let folder_path= get_folder_path(base_url,"klines");

    data_to_json(folder_path.as_str(), product_id, json.as_str()).unwrap_or_else(|_| panic!("Unable to store data in json file"));

    Ok(candle_data)
}

/// Retrieves candle data from a specified source.
/// The function takes as input the name of the source (either "api" or "file")
#[allow(dead_code)]
pub fn candles(
    source      : &str,
    base_url    : &str,
    granularity : u32,
    limit       : u16,
    product_id  : &str
) -> Result<Vec<Klines>, Box<dyn Error>> {
    match source {
        "api" => {
            let candles = products(ApiParams {
                base_url,
                product_id  : Some(product_id),
                resource    : Some("candles"),
                limit,
                granularity,
                params      : None,
            })?;

            let klines: Vec<Klines> = candles.into_iter().map(|candle| Klines {
                timestamp   : candle[0] as u64,
                price_low   : candle[1],
                price_high  : candle[2],
                price_open  : candle[3],
                price_close : candle[4],
                volume      : candle[5],
            }).collect();
            Ok(klines)
        }
        "file" => {
            let folder_path = get_folder_path(base_url, "klines");

            let data = data_from_json(folder_path.as_str(), product_id)?;
            let klines: Vec<Klines> = from_str(&data)?;
            Ok(klines)
        }
        _ => Err(Box::new(CustomError::new("Undefined source name".into())))
    }
}