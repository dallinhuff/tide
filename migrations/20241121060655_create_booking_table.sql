CREATE TABLE IF NOT EXISTS booking
(
    booking_id  UUID NOT NULL,
    customer_id UUID NOT NULL,
    trip_id     UUID NOT NULL,

    PRIMARY KEY (booking_id),
    FOREIGN KEY (customer_id) REFERENCES customer (customer_id),
    FOREIGN KEY (trip_id) REFERENCES trip (trip_id),
    UNIQUE (customer_id, trip_id)
);