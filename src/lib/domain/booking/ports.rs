use crate::domain::booking::models::booking::{Booking, BookingError, BookingFilters, BookingId};
use crate::domain::booking::models::customer::{Customer, CustomerError, CustomerId};
use crate::domain::booking::models::trip::{Trip, TripError, TripFilters, TripId};
use std::future::Future;

/// [BookingService] is able to handle use-case interactions with the booking domain.
pub trait BookingService: Clone + Send + Sync + 'static {
    fn find_booking(
        &self,
        id: BookingId,
    ) -> impl Future<Output = Result<Option<Booking>, BookingError>> + Send;
}

/// [BookingRepository] is able to access and persist booking domain models.
pub trait BookingRepository: Clone + Send + Sync + 'static {
    fn find_booking(
        &self,
        id: BookingId,
    ) -> impl Future<Output = Result<Option<Booking>, BookingError>> + Send;

    fn find_bookings(
        &self,
        filters: &BookingFilters,
    ) -> impl Future<Output = Result<Vec<Booking>, BookingError>> + Send;

    fn save_booking(
        &self,
        booking: &Booking,
    ) -> impl Future<Output = Result<(), BookingError>> + Send;

    fn delete_booking(
        &self,
        id: BookingId,
    ) -> impl Future<Output = Result<(), BookingError>> + Send;

    fn find_customer(
        &self,
        id: CustomerId,
    ) -> impl Future<Output = Result<Option<Customer>, CustomerError>> + Send;

    fn save_customer(
        &self,
        customer: &Customer,
    ) -> impl Future<Output = Result<(), CustomerError>> + Send;

    fn delete_customer(
        &self,
        id: CustomerId,
    ) -> impl Future<Output = Result<(), CustomerError>> + Send;

    fn find_trip(
        &self,
        id: TripId
    ) -> impl Future<Output = Result<Option<Trip>, TripError>> + Send;
    
    fn find_trips(
        &self,
        trip_filters: &TripFilters,
    ) -> impl Future<Output = Result<Vec<Trip>, TripError>> + Send;
}
