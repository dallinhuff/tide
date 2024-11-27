use crate::domain::booking::models::customer::CustomerId;
use crate::domain::booking::models::trip::TripId;
use crate::domain::booking::models::waiver::WaiverId;
use uuid::Uuid;

/// A [Booking] represents the intent for a group of [Participant]s to participate in a [Trip].
/// It includes the [Customer] who made the booking as well as the [Equipment] rentals they made.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Booking {
    pub id: BookingId,
    pub customer: CustomerId,
    pub trip: TripId,
    pub participants: Vec<Participant>,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BookingId(pub Uuid);

/// A [Participant] is a person who participates in a [Trip] as a member of a [Booking].
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Participant {
    pub id: ParticipantId,
    pub name: String,
    pub dob: chrono::NaiveDate,
    pub notes: String,
    pub waiver: Option<WaiverId>,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ParticipantId(pub Uuid);

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BookingFilters {
    pub customer: Option<CustomerId>,
    pub trip: Option<TripId>,
    pub participant: Option<ParticipantId>,
}

impl BookingFilters {
    pub fn is_empty(&self) -> bool {
        self.customer.is_none() && self.trip.is_none() && self.participant.is_none()
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CreateBookingRequest {
    customer_id: CustomerId,
    trip_id: Uuid,
}

pub enum BookingError {
    Unknown(anyhow::Error),
}
