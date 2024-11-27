use chrono::{DateTime, Utc};
use uuid::Uuid;

/// A [Trip] is a scheduled/available [TripKind] that customers may make bookings for.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Trip {
    pub id: TripId,
    pub kind: TripKind,
    pub location: LocationId,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TripId(pub Uuid);

/// A [TripKind] is a category/classification of trip, or a service that the company provides.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TripKind {
    pub id: TripKindId,
    pub name: String,
    pub description: String,
    pub guided: bool,
    pub meal_provided: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TripKindId(pub Uuid);

/// A [Location] is a departure location associated with a [Trip].
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Location {
    pub id: LocationId,
    pub name: LocationName,
    pub description: LocationDescription,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LocationId(pub Uuid);

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LocationName(pub String);

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LocationDescription(pub String);

pub struct TripFilters {
    pub kind: Option<TripKindId>,
    pub location: Option<LocationId>,
    pub date_range: Option<(DateTime<Utc>, DateTime<Utc>)>,
}

impl TripFilters {
    pub fn is_empty(&self) -> bool {
        self.kind.is_none() && self.location.is_none() && self.date_range.is_none()
    }
}

pub enum TripError {
    Unknown(anyhow::Error),
}
