// @generated automatically by Diesel CLI.

diesel::table! {
    blocklist (id) {
        id -> Int8,
        ip -> Cidr,
        version -> Int2,
        description -> Nullable<Text>,
    }
}
