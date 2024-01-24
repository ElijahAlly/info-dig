// @generated automatically by Diesel CLI.

diesel::table! {
    campaigns (campaign_id) {
        campaign_id -> Int4,
        organization_id -> Int4,
        title -> Text,
        description -> Text,
        status -> Text,
        goal_amount -> Numeric,
        current_amount -> Numeric,
        start_date -> Date,
        end_date -> Nullable<Date>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    organizations (organization_id) {
        organization_id -> Int4,
        name -> Text,
        description -> Nullable<Text>,
        contact_email -> Text,
        contact_phone -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    proposals (proposal_id) {
        proposal_id -> Int4,
        user_id -> Int4,
        slug -> Varchar,
        title -> Text,
        description -> Text,
        status -> Nullable<Text>,
        yeas -> Int4,
        nays -> Int4,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    statements (statement_id) {
        statement_id -> Int4,
        user_id -> Int4,
        slug -> Varchar,
        content -> Text,
        context -> Nullable<Text>,
        public_rating -> Nullable<Text>,
        our_rating -> Nullable<Text>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    users (user_id) {
        user_id -> Int4,
        email -> Text,
        phone_number -> Nullable<Text>,
        email_verified -> Bool,
        phone_verified -> Bool,
        auth_token -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    campaigns,
    organizations,
    proposals,
    statements,
    users,
);
