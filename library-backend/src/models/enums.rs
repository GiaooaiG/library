use serde::{Deserialize, Serialize};

// 引入schema中定义的枚举类型
use crate::schema::sql_types::UsersRoleEnum;

#[derive(Debug, Serialize, Deserialize, Clone, diesel::deserialize::FromSqlRow, diesel::sql_types::SqlType)]
#[diesel(sql_type = UsersRoleEnum)]
pub enum UsersRole {
    Admin,
    User,
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

impl<DB> diesel::deserialize::FromSql<UsersRoleEnum, DB> for UsersRole
where
    DB: diesel::backend::Backend,
    *const str: diesel::deserialize::FromSql<diesel::sql_types::Text, DB>,
{
    fn from_sql(bytes: DB::RawValue<'_>) -> diesel::deserialize::Result<Self> {
        let s = <*const str as diesel::deserialize::FromSql<diesel::sql_types::Text, DB>>::from_sql(bytes)?;
        let s = unsafe { &*s };
        match s {
            "admin" => Ok(UsersRole::Admin),
            "user" => Ok(UsersRole::User),
            _ => Ok(UsersRole::User),
        }
    }
}

impl<DB> diesel::serialize::ToSql<UsersRoleEnum, DB> for UsersRole
where
    DB: diesel::backend::Backend,
    str: diesel::serialize::ToSql<diesel::sql_types::Text, DB>,
{
    fn to_sql<'b>(&'b self, out: &mut diesel::serialize::Output<'b, '_, DB>) -> diesel::serialize::Result {
        let s = match self {
            UsersRole::Admin => "admin",
            UsersRole::User => "user",
        };
        s.to_sql(out)
    }
}