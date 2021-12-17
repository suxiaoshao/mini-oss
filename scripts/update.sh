# 配置命名空间
kubectl apply -f ./kubernetes/namespace.yaml

# 配置 graphql
kubectl apply -f ./kubernetes/graphql.yaml

# 配置 mongodb 
kubectl apply -f ./kubernetes/mongodb.yaml

# 配置 postgresql 
kubectl apply -f ./kubernetes/postgresql.yaml
