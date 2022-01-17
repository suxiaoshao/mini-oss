# 配置命名空间
kubectl apply -f ./kubernetes/namespace.yaml

# 配置 前端
kubectl apply -f ./kubernetes/packages/web.yaml

# 配置 configmap

kubectl apply -f ./kubernetes/configmap.yaml

# 配置 auth
kubectl apply -f ./kubernetes/packages/auth.yaml

# 配置 user
kubectl apply -f ./kubernetes/packages/user.yaml

# 配置 graphql
kubectl apply -f ./kubernetes/packages/graphql.yaml

# 配置 graphql
kubectl apply -f ./kubernetes/packages/core.yaml