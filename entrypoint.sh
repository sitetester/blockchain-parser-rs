#!/bin/bash

# run migrations
diesel migration run

# start the application
./target/release/eth-blockchain-parser-rs
