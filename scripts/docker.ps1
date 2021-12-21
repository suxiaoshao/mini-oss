docker build -t suxiaoshao/postgres -f ./docker/postgres.Dockerfile .;
docker tag suxiaoshao/postgres suxiaoshao/postgres:latest;
docker push suxiaoshao/postgres:latest;

docker build -t suxiaoshao/graphql -f ./docker/graphql.Dockerfile .;
docker tag suxiaoshao/graphql suxiaoshao/graphql:latest;
docker push suxiaoshao/graphql:latest;

docker build -t suxiaoshao/auth -f ./docker/auth.Dockerfile .;
docker tag suxiaoshao/auth suxiaoshao/auth:0.0.2;
docker push suxiaoshao/auth:0.0.2;