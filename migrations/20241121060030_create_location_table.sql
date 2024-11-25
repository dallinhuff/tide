CREATE TABLE IF NOT EXISTS location
(
    location_id UUID NOT NULL,
    name        TEXT NOT NULL,
    description TEXT NOT NULL DEFAULT '',

    PRIMARY KEY (location_id),
    UNIQUE (name)
);