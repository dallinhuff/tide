CREATE TABLE IF NOT EXISTS trip_kind
(
    trip_kind_id  UUID NOT NULL,
    name          TEXT NOT NULL,
    description   TEXT NOT NULL DEFAULT '',
    guided        BOOL NOT NULL DEFAULT FALSE,
    meal_provided BOOL NOT NULL DEFAULT FALSE,

    PRIMARY KEY (trip_kind_id),
    UNIQUE (name)
);