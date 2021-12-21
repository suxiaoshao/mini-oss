docker build -t suxiaoshao/postgres -f ./docker/postgres.Dockerfile .;

docker build -t suxiaoshao/graphql -f ./docker/graphql.Dockerfile .;

docker build -t suxiaoshao/auth -f ./docker/auth.Dockerfile .;

docker build -t suxiaoshao/test -f ./docker/test.Dockerfile .;