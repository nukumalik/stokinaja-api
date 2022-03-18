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
