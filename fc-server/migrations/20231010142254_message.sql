create table if not exists message (
    id integer primary key not null,
    user_id integer not null,
    room_id integer not null,
    content blob not null,
    created_at text not null
);