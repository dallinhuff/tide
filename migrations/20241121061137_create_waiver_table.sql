CREATE TABLE IF NOT EXISTS waiver
(
    waiver_id UUID NOT NULL,
    content   TEXT NOT NULL,

    PRIMARY KEY (waiver_id)
);