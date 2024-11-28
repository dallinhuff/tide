use crate::domain::booking::models::booking::{Booking, BookingError, BookingFilters, BookingId};
use crate::domain::booking::models::customer::{Customer, CustomerError, CustomerId};
use crate::domain::booking::models::trip::{Trip, TripError, TripFilters, TripId};
use std::future::Future;
use crate::domain::booking::models::equipment::{BookingRentals, EquipmentError};

/// [BookingService] is able to handle use-case interactions with the booking domain.
pub trait BookingService: Clone + Send + Sync + 'static {
    fn find_booking(
        &self,
        id: BookingId,
    ) -> impl Future<Output = Result<Option<Booking>, BookingError>> + Send;
}

/// [BookingRepository] is able to access and persist booking domain models.
pub trait BookingRepository: Clone + Send + Sync + 'static {
    /// find_booking gets a [Booking] by ID if it exists.
    fn find_booking(
        &self,
        id: BookingId,
    ) -> impl Future<Output = Result<Option<Booking>, BookingError>> + Send;

    /// find_bookings gets all [Booking]s that match a given set of filters/criteria.
    fn find_bookings(
        &self,
        filters: &BookingFilters,
    ) -> impl Future<Output = Result<Vec<Booking>, BookingError>> + Send;

    /// save_booking atomically saves a booking & its participants.
    fn save_booking(
        &self,
        booking: &Booking,
    ) -> impl Future<Output = Result<(), BookingError>> + Send;

    /// delete_booking atomically deletes a booking & its participants/rentals.
    fn delete_booking(
        &self,
        id: BookingId,
    ) -> impl Future<Output = Result<(), BookingError>> + Send;

    /// find_customer gets a [Customer] by ID if it exists.
    fn find_customer(
        &self,
        id: CustomerId,
    ) -> impl Future<Output = Result<Option<Customer>, CustomerError>> + Send;

    /// save_customer creates or updates a customer.
    fn save_customer(
        &self,
        customer: &Customer,
    ) -> impl Future<Output = Result<(), CustomerError>> + Send;

    /// delete_customer deletes a customer by id.
    fn delete_customer(
        &self,
        id: CustomerId,
    ) -> impl Future<Output = Result<(), CustomerError>> + Send;

    /// find_trip gets a [Trip] by ID if it exists.
    fn find_trip(
        &self,
        id: TripId
    ) -> impl Future<Output = Result<Option<Trip>, TripError>> + Send;

    /// find_trips gets all [Trip]s that match a given set of filters/criteria.
    fn find_trips(
        &self,
        trip_filters: &TripFilters,
    ) -> impl Future<Output = Result<Vec<Trip>, TripError>> + Send;
    
    /// find_booking_rentals gets the [BookingRentals] for a given booking.
    fn find_booking_rentals(
        &self,
        booking_id: BookingId,
    ) -> impl Future<Output = Result<BookingRentals, EquipmentError>> + Send;
    
    /// save_booking_rentals saves or updates all rentals for a booking.
    fn save_booking_rentals(
        &self,
        booking_rentals: &BookingRentals
    ) -> impl Future<Output = Result<(), EquipmentError>> + Send;
}
