CREATE TABLE IF NOT EXISTS booking_equipment
(
    booking_id   UUID NOT NULL,
    equipment_id UUID NOT NULL,
    quantity     INT  NOT NULL,

    PRIMARY KEY (booking_id, equipment_id),
    FOREIGN KEY (booking_id) REFERENCES booking (booking_id),
    FOREIGN KEY (equipment_id) REFERENCES equipment (equipment_id)
);
