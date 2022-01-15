\c mini_oss;
create table if not exists Bucket
(
    name        varchar(255) primary key not null unique,
    create_time timestamp                not null,
    update_time timestamp                not null,
    access      access_type              not null,
    user_name   varchar(25)              not null
);
create table if not exists BucketTag
(
    tag_name    varchar(25)  not null,
    bucket_name varchar(255) not null,
    value       varchar(255) not null,
    primary key (tag_name, bucket_name)
);