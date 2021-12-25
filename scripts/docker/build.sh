docker build -t suxiaoshao/postgres:latest -f ./docker/postgres.Dockerfile .;

docker build -t suxiaoshao/rust:latest -f ./docker/rust.Dockerfile .;

docker build -t suxiaoshao/graphql:0.0.6 -f ./docker/graphql.Dockerfile .;

docker build -t suxiaoshao/auth:0.0.5 -f ./docker/auth.Dockerfile .;

docker build -t suxiaoshao/test:0.0.1 -f ./docker/test.Dockerfile .;