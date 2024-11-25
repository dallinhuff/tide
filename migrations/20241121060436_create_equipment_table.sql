CREATE TABLE IF NOT EXISTS equipment
(
    equipment_id    UUID NOT NULL,
    name            TEXT NOT NULL,
    description     TEXT NOT NULL DEFAULT '',
    total_inventory INT  NOT NULL DEFAULT 0,

    PRIMARY KEY (equipment_id),
    UNIQUE (name)
);