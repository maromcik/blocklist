// @generated automatically by Diesel CLI.

diesel::table! {
    blocklist (id) {
        id -> Int8,
        ip -> Cidr,
        version -> Int2,
        country_code -> Nullable<Text>,
        isp -> Nullable<Text>,
        user_agent -> Nullable<Text>,
        description -> Nullable<Text>,
        added_at -> Timestamptz,
    }
}
