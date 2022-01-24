\c mini_oss;
CREATE TYPE bucket_access_type AS ENUM ('Open', 'ReadOpen', 'Private');
CREATE TYPE object_access_type AS ENUM ('Bucket', 'ReadOpen', 'Private');
create type header_type as
(
    key   text,
    value text
);