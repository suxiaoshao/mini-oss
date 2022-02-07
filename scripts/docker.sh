docker build -t suxiaoshao/postgres:latest -f ./docker/postgres.Dockerfile .;
docker push suxiaoshao/postgres:latest;

docker build -t suxiaoshao/rust:latest -f ./docker/rust.Dockerfile .;
docker push suxiaoshao/rust:latest;

docker build -t suxiaoshao/graphql:latest -f ./docker/packages/graphql.Dockerfile .;
docker push suxiaoshao/graphql:latest;

docker build -t suxiaoshao/auth:latest -f ./docker/packages/auth.Dockerfile .;
docker push suxiaoshao/auth:latest;

docker build -t suxiaoshao/user:latest -f ./docker/packages/user.Dockerfile .;
docker push suxiaoshao/user:latest;

docker build -t suxiaoshao/core:latest -f ./docker/packages/core.Dockerfile .;
docker push suxiaoshao/core:latest;

docker build -t suxiaoshao/open:latest -f ./docker/packages/open.Dockerfile .;
docker push suxiaoshao/open:latest;