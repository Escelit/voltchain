CREATE TABLE trades (
    id UUID PRIMARY KEY,
    prosumer_address VARCHAR(255) NOT NULL,
    consumer_address VARCHAR(255) NOT NULL,
    amount_kwh DOUBLE PRECISION NOT NULL,
    price_per_kwh DOUBLE PRECISION NOT NULL,
    timestamp TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);
