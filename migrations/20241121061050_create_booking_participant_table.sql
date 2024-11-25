CREATE TABLE IF NOT EXISTS booking_participant
(
    booking_id     UUID NOT NULL,
    participant_id UUID NOT NULL,

    PRIMARY KEY (booking_id, participant_id),
    FOREIGN KEY (booking_id) REFERENCES booking (booking_id),
    FOREIGN KEY (participant_id) REFERENCES participant (participant_id)
);