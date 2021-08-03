CREATE TABLE blocks (
    id                INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    difficulty        VARCHAR NOT NULL,
    hash              VARCHAR NOT NULL,
    number            INTEGER NOT NULL,
    transactionsCount INTEGER NOT NULL
);


CREATE TABLE transactions (
    id                INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
   blockHash          VARCHAR NOT NULL,
   blockNumber        VARCHAR NOT NULL,
   `from`             VARCHAR  NULL,
   hash               VARCHAR NOT NULL,
   `to`               VARCHAR  NULL,
   transactionIndex   VARCHAR NOT NULL
);