create table if not exists user (
    id integer primary key not null,
    name text not null,
    password text not null,
    created_at text not null
);