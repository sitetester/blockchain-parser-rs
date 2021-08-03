table! {
    blocks (id) {
        id -> Integer,
        difficulty -> Text,
        hash -> Text,
        number -> Integer,
        transactionsCount -> Integer,
    }
}

table! {
    transactions (id) {
        id -> Integer,
        blockHash -> Text,
        blockNumber -> Text,
        from -> Nullable<Text>,
        hash -> Text,
        to -> Nullable<Text>,
        transactionIndex -> Text,
    }
}

allow_tables_to_appear_in_same_query!(
    blocks,
    transactions,
);
