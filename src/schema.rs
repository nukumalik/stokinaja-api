table! {
    companies (id) {
        id -> Varchar,
        name -> Varchar,
        address -> Nullable<Text>,
        description -> Nullable<Text>,
        email -> Varchar,
        phone -> Varchar,
        created_at -> Timestamp,
        upadted_at -> Timestamp,
    }
}

table! {
    sellers (id) {
        id -> Varchar,
        name -> Varchar,
        address -> Nullable<Text>,
        email -> Varchar,
        password -> Varchar,
        phone -> Nullable<Varchar>,
        created_at -> Timestamp,
        upadted_at -> Timestamp,
    }
}

table! {
    suppliers (id) {
        id -> Varchar,
        name -> Varchar,
        email -> Varchar,
        password -> Varchar,
        phone -> Nullable<Varchar>,
        company_id -> Varchar,
        created_at -> Timestamp,
        upadted_at -> Timestamp,
    }
}

joinable!(suppliers -> companies (company_id));

allow_tables_to_appear_in_same_query!(
    companies,
    sellers,
    suppliers,
);
