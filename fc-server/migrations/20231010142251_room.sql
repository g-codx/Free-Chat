create table if not exists room (
    id integer primary key not null,
    name text not null,
    last_message text not null,
    user_ids text not null,
    created_at text not null
);
