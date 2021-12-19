\c mini_oss;
create table if not exists Users
(
    id          serial primary key not null,
    name        varchar(25)        not null,
    create_time timestamp          not null,
    update_time timestamp          not null,
    password    text               not null
);