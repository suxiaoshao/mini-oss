\c mini_oss;
drop type access_type;
CREATE TYPE access_type AS ENUM ('Open', 'ReadOpen', 'Private');