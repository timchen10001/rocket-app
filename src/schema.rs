// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Integer,
        name -> Text,
        email -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}
