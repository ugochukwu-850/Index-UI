// @generated automatically by Diesel CLI.

diesel::table! {
    destino_users (id) {
        id -> Uuid,
        fullname -> Text,
        email -> Text,
        phone_number -> Int8,
        joined -> Date,
    }
}
