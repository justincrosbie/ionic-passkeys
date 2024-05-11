// @generated automatically by Diesel CLI.

diesel::table! {
    applications (uuid) {
        uuid -> Uuid,
        name -> Varchar,
        displayname -> Varchar,
        state -> Varchar,
        tenant_uuid -> Uuid,
        apikey -> Varchar,
        created_at -> Timestamp,
        modified_at -> Nullable<Timestamp>,
    }
}

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
    passkeystates (uuid) {
        uuid -> Uuid,
        user_uuid -> Uuid,
        username -> Varchar,
        challenge -> Varchar,
        orig_challenge -> Varchar,
        state_data -> Text,
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
    tenants (uuid) {
        uuid -> Uuid,
        name -> Varchar,
        displayname -> Varchar,
        state -> Varchar,
        created_at -> Timestamp,
        modified_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    tenantusers (user_uuid) {
        user_uuid -> Uuid,
        tenant_uuid -> Uuid,
        role -> Varchar,
        created_at -> Timestamp,
        modified_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    users (uuid) {
        uuid -> Uuid,
        username -> Varchar,
        displayname -> Varchar,
        email -> Varchar,
        mobile -> Varchar,
        public_key -> Varchar,
        user_state -> Varchar,
        created_at -> Timestamp,
        modified_at -> Nullable<Timestamp>,
        verified -> Bool,
    }
}

diesel::joinable!(applications -> tenants (tenant_uuid));
diesel::joinable!(credentials -> users (user_uuid));
diesel::joinable!(tenantusers -> tenants (tenant_uuid));

diesel::allow_tables_to_appear_in_same_query!(
    applications,
    credentials,
    passkeystates,
    regstates,
    tenants,
    tenantusers,
    users,
);
