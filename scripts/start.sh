# 删除旧集群
kind delete cluster --name mini-oss;

# # 创建新集群
# kind create cluster --config=./kubernetes/config.yaml;

# # 配置 apisix ingress
# ./scripts/dev_hlem_update.sh
# kubectl create ns ingress-apisix
# helm install apisix apisix/apisix \
#   --set gateway.type=NodePort \
#   --set gateway.http.nodePort=30002 \
#   --set ingress-controller.enabled=true \
#   --namespace ingress-apisix
#   --set ingress-controller.config.apisix.serviceNamespace=ingress-apisix 

# kubectl get service --namespace ingress-apisix

# # novel 项目配置更新
# ./scripts/dev_novel_update.sh