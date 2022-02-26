\c mini_oss;
create table if not exists storage
(
    time        timestamp    not null,
    bucket_name varchar(255) not null,
    size        numeric      not null,
    num         bigint       not null,
    primary key (time, bucket_name)
);
create table if not exists request
(
    time          timestamp    not null,
    bucket_name   varchar(255) not null,
    upload_size   numeric      not null,
    download_size numeric      not null,
    num           bigint       not null,
    primary key (time, bucket_name)
);