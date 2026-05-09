// @generated automatically by Diesel CLI.

diesel::table! {
    ledger_cursors (contract_id) {
        id -> Uuid,
        contract_id -> Varchar,
        last_ledger -> Int8,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    trades (id) {
        id -> Uuid,
        prosumer_address -> Varchar,
        consumer_address -> Varchar,
        amount_kwh -> Float8,
        price_per_kwh -> Float8,
        timestamp -> Timestamp,
    }
}
