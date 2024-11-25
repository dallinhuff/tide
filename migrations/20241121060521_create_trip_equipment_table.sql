CREATE TABLE IF NOT EXISTS trip_equipment
(
    trip_kind_id UUID NOT NULL,
    equipment_id UUID NOT NULL,

    PRIMARY KEY (trip_kind_id, equipment_id),
    FOREIGN KEY (trip_kind_id) REFERENCES trip_kind (trip_kind_id),
    FOREIGN KEY (equipment_id) REFERENCES equipment (equipment_id)
);