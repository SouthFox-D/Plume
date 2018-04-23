table! {
    blogs (id) {
        id -> Int4,
        actor_id -> Varchar,
        title -> Varchar,
        summary -> Text,
        outbox_url -> Varchar,
        inbox_url -> Varchar,
        instance_id -> Int4,
    }
}

table! {
    instances (id) {
        id -> Int4,
        local_domain -> Varchar,
        public_domain -> Varchar,
        name -> Varchar,
        local -> Bool,
        blocked -> Bool,
    }
}

table! {
    users (id) {
        id -> Int4,
        username -> Varchar,
        display_name -> Varchar,
        outbox_url -> Varchar,
        inbox_url -> Varchar,
        is_admin -> Bool,
        summary -> Text,
        email -> Nullable<Text>,
        hashed_password -> Nullable<Text>,
        instance_id -> Int4,
    }
}

joinable!(blogs -> instances (instance_id));
joinable!(users -> instances (instance_id));

allow_tables_to_appear_in_same_query!(
    blogs,
    instances,
    users,
);
