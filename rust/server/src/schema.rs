// @generated automatically by Diesel CLI.

diesel::table! {
    credentials (uuid) {
        uuid -> Uuid,
        user_uuid -> Uuid,
        credentialid -> Varchar,
        passkey -> Text,
        created_at -> Timestamp,
        modified_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    regstates (uuid) {
        uuid -> Uuid,
        user_uuid -> Uuid,
        username -> Varchar,
        challenge -> Varchar,
        reg_state -> Text,
        created_at -> Timestamp,
        modified_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    users (uuid) {
        uuid -> Uuid,
        username -> Varchar,
        email -> Varchar,
        mobile -> Varchar,
        public_key -> Varchar,
        user_state -> Varchar,
        created_at -> Timestamp,
        modified_at -> Nullable<Timestamp>,
        verified -> Bool,
    }
}

diesel::joinable!(credentials -> users (user_uuid));

diesel::allow_tables_to_appear_in_same_query!(
    credentials,
    regstates,
    users,
);
