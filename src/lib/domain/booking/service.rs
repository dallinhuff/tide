use crate::domain::booking::models::booking::{Booking, BookingError, BookingId};
use crate::domain::booking::ports::{BookingRepository, BookingService};

#[derive(Debug, Clone)]
pub struct Service<R: BookingRepository> {
    repo: R,
}

impl<R: BookingRepository> Service<R> {
    pub fn new(repo: R) -> Self {
        Self { repo }
    }
}

impl<R: BookingRepository> BookingService for Service<R> {
    async fn find_booking(&self, id: BookingId) -> Result<Option<Booking>, BookingError> {
        self.repo.find_booking(id).await
    }
}
