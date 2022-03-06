\c mini_oss;
create table if not exists Users
(
    name        varchar(25) primary key not null unique,
    create_time timestamptz             not null,
    update_time timestamptz             not null,
    password    text                    not null,
    description text
);