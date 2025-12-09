use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum UsersRole {
    Admin,
    User,
}

impl UsersRole {
    pub fn to_str(&self) -> &'static str {
        match self {
            UsersRole::Admin => "admin",
            UsersRole::User => "user",
        }
    }
    
    pub fn from_str(s: &str) -> Self {
        match s {
            "admin" => UsersRole::Admin,
            _ => UsersRole::User,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum BorrowStatus {
    Borrowed,
    Returned,
    Overdue,
}

impl BorrowStatus {
    pub fn to_str(&self) -> &'static str {
        match self {
            BorrowStatus::Borrowed => "borrowed",
            BorrowStatus::Returned => "returned",
            BorrowStatus::Overdue => "overdue",
        }
    }
    
    pub fn from_str(s: &str) -> Self {
        match s {
            "returned" => BorrowStatus::Returned,
            "overdue" => BorrowStatus::Overdue,
            _ => BorrowStatus::Borrowed,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ReservationStatus {
    Pending,
    Fulfilled,
    Cancelled,
}

impl ReservationStatus {
    pub fn to_str(&self) -> &'static str {
        match self {
            ReservationStatus::Pending => "pending",
            ReservationStatus::Fulfilled => "fulfilled",
            ReservationStatus::Cancelled => "cancelled",
        }
    }
    
    pub fn from_str(s: &str) -> Self {
        match s {
            "fulfilled" => ReservationStatus::Fulfilled,
            "cancelled" => ReservationStatus::Cancelled,
            _ => ReservationStatus::Pending,
        }
    }
}