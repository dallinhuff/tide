CREATE TABLE IF NOT EXISTS participant_waiver
(
    participant_waiver_id UUID NOT NULL,
    participant_id        UUID NOT NULL,
    waiver_id             UUID NOT NULL,
    date_signed           DATE NOT NULL,

    PRIMARY KEY (participant_waiver_id),
    FOREIGN KEY (participant_id) REFERENCES participant (participant_id),
    FOREIGN KEY (waiver_id) REFERENCES waiver (waiver_id)
);