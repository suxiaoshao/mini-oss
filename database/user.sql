\c mini_oss;
create table if not exists Users
(
    name        varchar(25) primary key not null unique,
    create_time timestamp               not null,
    update_time timestamp               not null,
    password    text                    not null,
    description text
);