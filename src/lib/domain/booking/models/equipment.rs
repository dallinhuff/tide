use std::collections::HashMap;
use uuid::Uuid;
use crate::domain::booking::models::booking::BookingId;

/// [Equipment] is an item that is included with (or can be rented for) a [Booking].
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Equipment {
    pub id: EquipmentId,
    pub name: EquipmentName,
    pub description: EquipmentDescription,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EquipmentId(pub Uuid);

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EquipmentName(pub String);

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EquipmentDescription(pub String);

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BookingRentals {
    pub booking_id: BookingId,
    pub rentals: HashMap<EquipmentId, i32>
}

pub enum EquipmentError {
    Unknown(anyhow::Error),
}