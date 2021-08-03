#![allow(non_snake_case)]
#[macro_use]
extern crate diesel;
extern crate dotenv;

use diesel::prelude::*;
use diesel::SqliteConnection;
use dotenv::dotenv;
use tokio::spawn;

use eth_client::EthClient;

use crate::models::EthTransaction;
use crate::models::NewEthBlock;
use crate::schema::blocks;
use crate::schema::transactions;
use std::env;

mod eth_client;
pub mod models;
pub mod schema;

// https://rust-classes.com/chapter_5_2.html
#[tokio::main]
async fn main() {
    let cn = establishConnection();

    let mut start = 1;
    let mut end = start + 49;

    loop
    {
        manageImport(start, end, &cn).await;
        start = end + 1;
        end += 50;
    }
}

async fn manageImport(start: i32, num: i32, cn: &SqliteConnection) {
    print!("{} - {}", start, num);

    let mut tasks = vec![];

    for n in start..=num {
        let t = spawn(async move {
            EthClient::getBlockByNumber(n).await.unwrap()
        });

        tasks.push(t);
    }

    let mut blocks: Vec<NewEthBlock> = vec![];
    let mut ethTransactions: Vec<Vec<EthTransaction>> = vec![];

    for task in tasks {
        let blockByNumberResponse = task.await.expect("task failed");
        let ethBlock = blockByNumberResponse.result.unwrap();
        let hex = ethBlock.number;

        let newEthBlock: NewEthBlock = NewEthBlock {
            difficulty: ethBlock.difficulty,
            hash: ethBlock.hash,
            number: i64::from_str_radix(hex.trim_start_matches("0x"), 16).unwrap() as i32,
            transactionsCount: ethBlock.transactions.len() as i32,
        };

        blocks.push(newEthBlock);
        ethTransactions.push(ethBlock.transactions);
    }

    diesel::insert_into(blocks::table)
        .values(&blocks)
        .execute(cn)
        .expect("Error saving new blocks!");

    for tx in ethTransactions {
        diesel::insert_into(transactions::table)
            .values(&tx)
            .execute(cn)
            .expect("Error saving new tx!");
    }

    println!(" ☑️️")
}

pub fn establishConnection() -> SqliteConnection {
    dotenv().ok();

    let dbUrl = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    SqliteConnection::establish(&dbUrl)
        .expect(&format!("Error connecting to {}", dbUrl))
}

