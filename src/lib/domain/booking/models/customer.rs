use thiserror::Error;
use uuid::Uuid;

/// A [Customer] is a person who makes a [Booking] (either in advance or as a walk-in)
/// and is financially responsible for and the primary contact of their group of participants.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Customer {
    pub id: CustomerId,
    pub name: CustomerName,
    pub email: EmailAddress,
    pub phone: PhoneNumber,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CustomerId(pub Uuid);

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CustomerName(pub String);

impl TryFrom<&String> for CustomerName {
    type Error = CustomerError;

    fn try_from(str: &String) -> Result<Self, Self::Error> {
        let trimmed = str.trim();
        if trimmed.len() <= 2 {
            Err(CustomerError::InvalidName(trimmed.to_owned()))
        } else {
            Ok(Self(trimmed.to_owned()))
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EmailAddress(pub String);

impl TryFrom<&String> for EmailAddress {
    type Error = CustomerError;

    fn try_from(str: &String) -> Result<Self, Self::Error> {
        let trimmed = str.trim();
        if trimmed.len() <= 2 {
            Err(CustomerError::InvalidEmail(trimmed.to_owned()))
        } else {
            Ok(Self(trimmed.to_owned()))
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PhoneNumber(pub String);

impl TryFrom<&String> for PhoneNumber {
    type Error = CustomerError;

    fn try_from(str: &String) -> Result<Self, Self::Error> {
        let trimmed = str.trim();
        if trimmed.len() <= 2 {
            Err(CustomerError::InvalidPhone(trimmed.to_owned()))
        } else {
            Ok(Self(trimmed.to_owned()))
        }
    }
}

pub struct CreateCustomerRequest {
    pub name: String,
    pub email: String,
    pub phone: String,
}

impl TryFrom<CreateCustomerRequest> for Customer {
    type Error = CustomerError;

    fn try_from(request: CreateCustomerRequest) -> Result<Self, Self::Error> {
        Ok(Self{
            id: CustomerId(Uuid::now_v7()),
            name: CustomerName::try_from(&request.name)?,
            email: EmailAddress::try_from(&request.email)?,
            phone: PhoneNumber::try_from(&request.phone)?,
        })
    }
}

pub struct EditCustomerRequest {
    pub id: CustomerId,
    pub name: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
}

#[derive(Debug, Error)]
pub enum CustomerError {
    #[error("\"{0}\" is not a valid name")]
    InvalidName(String),
    #[error("\"{0}\" is not a valid email address")]
    InvalidEmail(String),
    #[error("A customer with email \"{0}\" already exists")]
    EmailTaken(String),
    #[error("\"{0}\" is not a valid phone number")]
    InvalidPhone(String),
    #[error(transparent)]
    Unknown(anyhow::Error)
}