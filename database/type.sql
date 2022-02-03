\c mini_oss;
CREATE TYPE bucket_access_type AS ENUM ('Open', 'ReadOpen', 'Private');
CREATE TYPE folder_access_type AS ENUM ('Open', 'ReadOpen', 'Private','Inheritance');
CREATE TYPE object_access_type AS ENUM ('Inheritance', 'ReadOpen', 'Private');