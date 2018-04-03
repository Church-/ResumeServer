table! {
    projects {
        name -> Varchar,
        body -> Text,
        link -> Varchar,
        file_link -> Varchar,
    }
}

table! {
    posts {
        id -> Nullable<Integer>,
        name -> Varchar,
        body -> Text,
        link -> Varchar,
        Date -> Varchar,
    }
}