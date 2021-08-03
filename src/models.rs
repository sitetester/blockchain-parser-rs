use serde::{Deserialize};

use crate::schema::*;

#[derive(Debug, Deserialize)]
pub struct EthBlock {
    pub difficulty: String,
    pub hash: String,
    pub number: String,
    pub transactions: Vec<EthTransaction>,
}

#[derive(Debug, Deserialize, Queryable, Insertable)]
#[table_name = "blocks"]
pub struct NewEthBlock {
    pub difficulty: String,
    pub hash: String,
    pub number: i32,
    pub transactionsCount: i32,
}


#[derive(Debug, Deserialize, Queryable, Insertable)]
#[table_name = "transactions"]
pub struct EthTransaction {
    pub blockHash: String,
    pub blockNumber: String,
    pub from: String,
    pub hash: String,
    pub to: Option<String>,
    pub transactionIndex: String,
}