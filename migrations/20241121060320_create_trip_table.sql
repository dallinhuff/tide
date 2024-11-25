CREATE TABLE IF NOT EXISTS trip
(
    trip_id      UUID        NOT NULL,
    trip_kind_id UUID        NOT NULL,
    location_id  UUID        NOT NULL,
    start_time   TIMESTAMPTZ NOT NULL,
    end_time     TIMESTAMPTZ NOT NULL,

    PRIMARY KEY (trip_id),
    FOREIGN KEY (trip_kind_id) REFERENCES trip_kind (trip_kind_id),
    FOREIGN KEY (location_id) REFERENCES location (location_id)
);