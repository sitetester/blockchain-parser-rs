use std::collections::HashMap;

use serde::{Deserialize};
use serde_json::{json};
use crate::models::EthBlock;


#[derive(Debug, Deserialize)]
pub struct BlockByNumberResponse {
    pub jsonrpc: String,
    pub id: String,
    pub result: Option<EthBlock>
}

const URL: &str = "https://mainnet.infura.io/v3/c8d36b72d2d04f16a94931809cdf6383";

pub struct EthClient {}

impl EthClient {
    // https://infura.io/docs/ethereum#operation/eth_getBlockByNumber
    // https://docs.rs/reqwest/0.11.4/reqwest/struct.Response.html#method.json
    // https://docs.serde.rs/serde_json/enum.Value.html#variant.Array
    pub async fn getBlockByNumber(number: i32) -> Result<BlockByNumberResponse, Box<dyn std::error::Error>> {
        let hexNum = format!("0x{:X}", number);

        let client = reqwest::Client::new();
        let mut params = HashMap::new();
        params.insert(hexNum.to_string(), true);

        let blockByNumberRequest = json!({"id": "1", "jsonrpc": "2.0", "method": "eth_getBlockByNumber", "params": json!([hexNum, true])});

        let response = client.post(URL).json(&blockByNumberRequest).send().await?;
        // println!("***************{:#?}", &response.text().await?);

        let blockByNumberResponse = response.json::<BlockByNumberResponse>().await?;
        Ok(blockByNumberResponse)

    }
}

