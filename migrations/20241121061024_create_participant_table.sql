CREATE TABLE IF NOT EXISTS participant
(
    participant_id UUID NOT NULL,
    name           TEXT NOT NULL,
    dob            DATE NOT NULL,
    notes          TEXT NOT NULL DEFAULT '',

    PRIMARY KEY (participant_id)
);