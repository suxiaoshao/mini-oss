SELECT 'create database mini_oss'
WHERE NOT EXISTS (SELECT FROM pg_database WHERE datname = 'mini_oss')\gexec