use crate::domain::booking::models::booking::*;
use crate::domain::booking::models::customer::*;
use crate::domain::booking::models::trip::*;
use crate::domain::booking::models::waiver::*;
use crate::domain::booking::ports::BookingRepository;
use crate::outbound::postgres::Postgres;
use chrono::{DateTime, NaiveDate, Utc};
use sqlx::{query, query_as, FromRow, QueryBuilder};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(FromRow, Debug)]
struct BookingDto {
    booking_id: Uuid,
    customer_id: Uuid,
    trip_id: Uuid,
    participant_id: Uuid,
    name: String,
    dob: NaiveDate,
    notes: String,
    waiver_id: Option<Uuid>,
}

struct CustomerDto {
    customer_id: Uuid,
    name: String,
    email: String,
    phone: String,
}

impl From<CustomerDto> for Customer {
    fn from(dto: CustomerDto) -> Self {
        Self {
            id: CustomerId(dto.customer_id),
            name: CustomerName(dto.name),
            email: EmailAddress(dto.email),
            phone: PhoneNumber(dto.phone),
        }
    }
}

#[derive(FromRow, Debug)]
struct TripDto {
    trip_id: Uuid,
    trip_kind_id: Uuid,
    name: String,
    description: String,
    guided: bool,
    meal_provided: bool,
    location_id: Uuid,
    start_time: DateTime<Utc>,
    end_time: DateTime<Utc>,
}

impl From<TripDto> for Trip {
    fn from(dto: TripDto) -> Self {
        Self {
            id: TripId(dto.trip_id),
            kind: TripKind {
                id: TripKindId(dto.trip_kind_id),
                name: dto.name,
                description: dto.description,
                guided: dto.guided,
                meal_provided: dto.meal_provided,
            },
            location: LocationId(dto.location_id),
            start_time: dto.start_time,
            end_time: dto.end_time,
        }
    }
}

impl From<sqlx::Error> for BookingError {
    fn from(error: sqlx::Error) -> Self {
        Self::Unknown(error.into())
    }
}

impl From<sqlx::Error> for CustomerError {
    fn from(error: sqlx::Error) -> Self {
        Self::Unknown(error.into())
    }
}

impl From<sqlx::Error> for TripError {
    fn from(error: sqlx::Error) -> Self {
        Self::Unknown(error.into())
    }
}

impl BookingRepository for Postgres {
    async fn find_booking(&self, id: BookingId) -> Result<Option<Booking>, BookingError> {
        let query = query_as!(
            BookingDto,
            // language=postgresql
            "SELECT
                booking_id,
                customer_id,
                trip_id,
                participant_id,
                name,
                dob,
                notes,
                waiver_id
             FROM booking
                JOIN booking_participant USING (booking_id)
                JOIN participant USING (participant_id)
                LEFT JOIN participant_waiver USING (participant_id)
             WHERE booking_id = $1",
            id.0
        );

        let results = query.fetch_all(&self.pool).await?;
        let Some(first) = results.first() else {
            return Ok(None);
        };

        Ok(Some(Booking {
            id: BookingId(first.booking_id),
            customer: CustomerId(first.customer_id),
            trip: TripId(first.trip_id),
            participants: results
                .into_iter()
                .map(|r| Participant {
                    id: ParticipantId(r.participant_id),
                    name: r.name,
                    dob: r.dob,
                    notes: r.notes,
                    waiver: r.waiver_id.map(|id| WaiverId(id)),
                })
                .collect(),
        }))
    }

    async fn find_bookings(&self, filters: &BookingFilters) -> Result<Vec<Booking>, BookingError> {
        if filters.is_empty() {
            // TODO: should actually return an error
            return Ok(vec![]);
        }

        // language=postgresql
        let query = "
            SELECT
                booking_id,
                customer_id,
                trip_id,
                participant_id,
                name,
                dob,
                notes,
                waiver_id
            FROM booking
               JOIN booking_participant USING (booking_id)
               JOIN participant USING (participant_id)
               LEFT JOIN participant_waiver USING (participant_id)
            WHERE TRUE
        ";

        let mut qb = sqlx::QueryBuilder::<sqlx::Postgres>::new(query);

        if let Some(CustomerId(id)) = filters.customer {
            qb.push(" AND customer_id = ").push_bind(id);
        }
        if let Some(TripId(id)) = filters.trip {
            qb.push(" AND trip_id = ").push_bind(id);
        }
        if let Some(ParticipantId(id)) = filters.participant {
            qb.push(" AND participant_id = ").push_bind(id);
        }

        qb.push(" ORDER BY booking_id ");

        let result = qb
            .build_query_as::<BookingDto>()
            .fetch_all(&self.pool)
            .await?;

        let mut bookings = HashMap::<BookingId, Booking>::new();

        for dto in result {
            let id = BookingId(dto.booking_id);
            let participant = Participant {
                id: ParticipantId(dto.participant_id),
                name: dto.name,
                dob: dto.dob,
                notes: dto.notes,
                waiver: dto.waiver_id.map(WaiverId),
            };

            if let Some(booking) = bookings.get_mut(&id) {
                booking.participants.push(participant);
            } else {
                bookings.insert(
                    id.clone(),
                    Booking {
                        id,
                        customer: CustomerId(dto.customer_id),
                        trip: TripId(dto.trip_id),
                        participants: vec![participant],
                    },
                );
            }
        }

        Ok(bookings.into_values().collect())
    }

    async fn save_booking(&self, booking: &Booking) -> Result<(), BookingError> {
        let (ids, names, dobs, notes) = participants_to_tuples(&booking.participants);

        let mut txn = self.pool.begin().await?;
        for command in [
            query!(
                // language=postgresql
                "INSERT INTO booking (booking_id, customer_id, trip_id)
                 VALUES ($1, $2, $3)",
                booking.id.0,
                booking.customer.0,
                booking.trip.0
            ),
            query!(
                // language=postgresql
                "INSERT INTO participant (participant_id, name, dob, notes)
                 SELECT * FROM UNNEST($1::UUID[], $2::TEXT[], $3::DATE[], $4::TEXT[])
                 ON CONFLICT (participant_id)
                 DO UPDATE SET
                    name = EXCLUDED.name,
                    dob = EXCLUDED.dob,
                    notes = EXCLUDED.notes",
                &ids,
                &names,
                &dobs,
                &notes,
            ),
            query!(
                // language=postgresql
                "DELETE FROM booking_participant WHERE booking_id = $1",
                booking.id.0
            ),
            query!(
                // language=postgresql
                "INSERT INTO booking_participant (booking_id, participant_id)
                 SELECT * FROM UNNEST($1::UUID[], $2::UUID[])",
                &vec![booking.id.0; booking.participants.len()],
                &booking
                    .participants
                    .iter()
                    .map(|p| p.id.0)
                    .collect::<Vec<_>>(),
            ),
        ] {
            command.execute(&mut *txn).await?;
        }
        txn.commit().await?;

        Ok(())
    }

    async fn delete_booking(&self, id: BookingId) -> Result<(), BookingError> {
        let mut txn = self.pool.begin().await?;
        for command in [
            query!(
                // language=postgresql
                "DELETE FROM booking_participant WHERE booking_id = $1",
                id.0
            ),
            query!(
                // language=postgresql
                "DELETE FROM booking WHERE booking_id = $1",
                id.0
            ),
        ] {
            command.execute(&mut *txn).await?;
        }
        txn.commit().await?;

        Ok(())
    }

    async fn find_customer(&self, id: CustomerId) -> Result<Option<Customer>, CustomerError> {
        let result = query_as!(
            CustomerDto,
            // language=postgresql
            "SELECT * FROM customer WHERE customer_id = $1",
            id.0
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(result.map(Customer::from))
    }

    async fn save_customer(&self, customer: &Customer) -> Result<(), CustomerError> {
        query!(
            // language=postgresql
            "INSERT INTO customer (customer_id, name, email, phone) VALUES ($1, $2, $3, $4)",
            customer.id.0,
            customer.name.0,
            customer.email.0,
            customer.phone.0
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn delete_customer(&self, id: CustomerId) -> Result<(), CustomerError> {
        query!(
            // language=postgresql
            "DELETE FROM customer WHERE customer_id = $1",
            id.0
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn find_trip(&self, id: TripId) -> Result<Option<Trip>, TripError> {
        let result = query_as!(
            TripDto,
            // language=postgresql
            "SELECT * 
             FROM trip JOIN trip_kind USING (trip_kind_id)
             WHERE trip_id = $1",
            id.0
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(result.map(Trip::from))
    }

    async fn find_trips(&self, trip_filters: &TripFilters) -> Result<Vec<Trip>, TripError> {
        if trip_filters.is_empty() {
            // TODO: should return error
            return Ok(vec![]);
        }
        
        // language=postgresql
        let query = "
            SELECT *
            FROM trip JOIN trip_kind USING (trip_kind_id)
            WHERE true
        ";

        let mut qb = QueryBuilder::<sqlx::Postgres>::new(query);

        if let Some(TripKindId(id)) = trip_filters.kind {
            qb.push(" AND trip_kind_id = ").push_bind(id);
        }
        if let Some(LocationId(id)) = trip_filters.location {
            qb.push(" AND location_id = ").push_bind(id);
        }
        if let Some((start, end)) = trip_filters.date_range {
            qb.push(" AND start_time BETWEEN ")
                .push_bind(start)
                .push(" AND ")
                .push_bind(end);
        }

        let result = qb.build_query_as::<TripDto>().fetch_all(&self.pool).await?;
        
        Ok(result.into_iter().map(Trip::from).collect())
    }
}

fn participants_to_tuples(
    participants: &Vec<Participant>,
) -> (Vec<Uuid>, Vec<String>, Vec<NaiveDate>, Vec<String>) {
    participants
        .iter()
        .fold((vec![], vec![], vec![], vec![]), |mut acc, participant| {
            acc.0.push(participant.id.0);
            acc.1.push(participant.name.to_string());
            acc.2.push(participant.dob);
            acc.3.push(participant.notes.to_string());
            acc
        })
}
