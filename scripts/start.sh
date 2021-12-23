# 删除旧集群
kind delete cluster --name mini-oss;

# 创建新集群
kind create cluster --config=./kubernetes/config.yaml;

# 配置 apisix ingress
helm repo add apisix https://charts.apiseven.com
helm repo add bitnami https://charts.bitnami.com/bitnami
helm repo update
kubectl create ns ingress-apisix

helm install apisix apisix/apisix \
  --set gateway.type=NodePort \
  --set gateway.http.nodePort=30002 \
  --set ingress-controller.enabled=true \
  --namespace ingress-apisix \
  --set ingress-controller.config.apisix.serviceNamespace=ingress-apisix 

kubectl get service --namespace ingress-apisix

# mini-oss 项目配置更新
./scripts/update.sh