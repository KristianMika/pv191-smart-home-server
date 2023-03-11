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
