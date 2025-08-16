// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(mysql_type(name = "Enum"))]
    pub struct BorrowRecordsStatusEnum;

    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(mysql_type(name = "Enum"))]
    pub struct ReservationsStatusEnum;

    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(mysql_type(name = "Enum"))]
    pub struct UsersRoleEnum;
}

diesel::table! {
    books (id) {
        id -> Integer,
        #[max_length = 13]
        isbn -> Varchar,
        #[max_length = 255]
        title -> Varchar,
        #[max_length = 255]
        author -> Varchar,
        #[max_length = 100]
        category -> Nullable<Varchar>,
        #[max_length = 255]
        publisher -> Nullable<Varchar>,
        total_copies -> Nullable<Integer>,
        available_copies -> Nullable<Integer>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::BorrowRecordsStatusEnum;

    borrow_records (id) {
        id -> Integer,
        user_id -> Integer,
        book_id -> Integer,
        borrow_date -> Nullable<Timestamp>,
        due_date -> Timestamp,
        return_date -> Nullable<Timestamp>,
        #[max_length = 8]
        status -> Nullable<BorrowRecordsStatusEnum>,
        renewal_count -> Nullable<Integer>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::ReservationsStatusEnum;

    reservations (id) {
        id -> Integer,
        user_id -> Integer,
        book_id -> Integer,
        reservation_date -> Nullable<Timestamp>,
        #[max_length = 9]
        status -> Nullable<ReservationsStatusEnum>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::UsersRoleEnum;

    users (id) {
        id -> Integer,
        #[max_length = 50]
        username -> Varchar,
        #[max_length = 255]
        password_hash -> Varchar,
        #[max_length = 6]
        role -> Nullable<UsersRoleEnum>,
        #[max_length = 20]
        phone -> Nullable<Varchar>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::joinable!(borrow_records -> books (book_id));
diesel::joinable!(borrow_records -> users (user_id));
diesel::joinable!(reservations -> books (book_id));
diesel::joinable!(reservations -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    books,
    borrow_records,
    reservations,
    users,
);
