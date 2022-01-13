docker build -t suxiaoshao/postgres:latest -f ./docker/postgres.Dockerfile .;
docker push suxiaoshao/postgres:latest;

docker build -t suxiaoshao/rust:latest -f ./docker/rust.Dockerfile .;
docker push suxiaoshao/rust:latest;

docker build -t suxiaoshao/graphql:latest -f ./docker/graphql.Dockerfile .;
docker push suxiaoshao/graphql:latest;

docker build -t suxiaoshao/auth:latest -f ./docker/auth.Dockerfile .;
docker push suxiaoshao/auth:latest;

docker build -t suxiaoshao/user:latest -f ./docker/user.Dockerfile .;
docker push suxiaoshao/user:latest;

docker build -t suxiaoshao/test:latest -f ./docker/test.Dockerfile .;
docker push suxiaoshao/test:latest;