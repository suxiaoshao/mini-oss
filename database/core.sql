\c mini_oss;
create table if not exists bucket
(
    name        varchar(255) primary key not null unique,
    create_time timestamptz              not null,
    update_time timestamptz              not null,
    access      bucket_access_type       not null,
    username    varchar(25)              not null
);
create table if not exists folder
(
    path        text               not null,
    create_time timestamptz        not null,
    update_time timestamptz        not null,
    bucket_name varchar(255)       not null,
    access      folder_access_type not null,
    father_path text               not null,
    primary key (path, bucket_name)
);
create table if not exists object
(
    username    varchar(25)        not null,
    bucket_name varchar(255)       not null,
    path        text               not null,
    filename    text               not null,
    create_time timestamptz        not null,
    update_time timestamptz        not null,
    object_id   varchar(24)        not null,
    blake3      varchar(64)        not null,
    size        numeric            not null,
    access      object_access_type not null,
    headers     jsonb[]            not null,
    primary key (path, filename, bucket_name)
);