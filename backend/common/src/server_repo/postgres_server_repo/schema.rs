// @generated automatically by Diesel CLI.

diesel::table! {
    use diesel::sql_types::*;

    measurement (id) {
        id -> Int4,
        temperature -> Nullable<Float4>,
        humidity -> Nullable<Int4>,
        voc_index -> Nullable<Int4>,
        measurement_time -> Timestamptz,
    }
}

diesel::table! {
    use diesel::sql_types::*;

    usercontext (id) {
        id -> Int4,
        first_name -> Varchar,
        user_login -> Varchar,
        user_password_hash -> Varchar,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    measurement,
    usercontext,
);
