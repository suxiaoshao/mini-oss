\c mini_oss;
create table if not exists storage
(
    time        timestamptz  not null,
    bucket_name varchar(255) not null,
    username    varchar(25)  not null,
    size        numeric      not null,
    num         bigint       not null,
    primary key (time, bucket_name)
);
create table if not exists request
(
    object_id     varchar(24) primary key not null unique,
    time          timestamptz             not null,
    bucket_name   varchar(255)            not null,
    username      varchar(25)             not null,
    upload_size   numeric                 not null,
    download_size numeric                 not null
);