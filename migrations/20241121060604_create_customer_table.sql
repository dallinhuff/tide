CREATE TABLE IF NOT EXISTS customer
(
    customer_id UUID NOT NULL,
    name        TEXT NOT NULL,
    email       TEXT NOT NULL,
    phone       TEXT NOT NULL,

    PRIMARY KEY (customer_id),
    UNIQUE (email)
);