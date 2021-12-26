# 配置命名空间
kubectl apply -f ./kubernetes/namespace.yaml

# 配置 configmap

kubectl apply -f ./kubernetes/configmap.yaml

# 配置 auth
kubectl apply -f ./kubernetes/auth.yaml

# 配置 user_manage
kubectl apply -f ./kubernetes/user_manage.yaml

# 配置 graphql
kubectl apply -f ./kubernetes/graphql.yaml