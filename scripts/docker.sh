docker build -t suxiaoshao/postgres:latest -f ./docker/postgres.Dockerfile .;
docker push suxiaoshao/postgres:latest;

docker build -t suxiaoshao/rust:latest -f ./docker/rust.Dockerfile .;
docker push suxiaoshao/rust:latest;

docker build -t suxiaoshao/graphql:0.0.8 -f ./docker/graphql.Dockerfile .;
docker push suxiaoshao/graphql:0.0.8;

docker build -t suxiaoshao/auth:0.0.7 -f ./docker/auth.Dockerfile .;
docker push suxiaoshao/auth:0.0.7;

docker build -t suxiaoshao/test:0.0.1 -f ./docker/test.Dockerfile .;
docker push suxiaoshao/test:0.0.1;