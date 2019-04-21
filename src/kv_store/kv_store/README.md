# Rust KVstore

## Docker

### Build
```sh
docker build -t sinvalvm/kvstore .
```

### Run
```sh
docker run -p 0.0.0.0:8000:8000 -d sinvalvm/kvstore
```

### Tag
```sh
docker tag sinvalvm/kvstore sinvalvm/kvstore:<tag-name>
```

### Dockerhub Push
```sh
docker push sinvalvm/kvstore:<tag-name>
```

### Pull
```sh
docker image pull sinvalvm/kvstore:<tag-name>
```

## Kubernetes

### Deploy k8s
```sh
kubectl create namespace kvstore
kubectl label namespace kvstore istio-injection=enabled
kubectl apply -f k8s/kvstore-deployment-service-1.0.yaml -n kvstore
kubectl apply -f k8s/kvstore-deployment-service-2.0.yaml -n kvstore
```

### Deploy istio
```sh
istioctl create -f k8s/istio/kvstore-istio-1.0.yaml --namespace kvstore
istioctl create -f k8s/istio/kvstore-istio-destinationrule-1.0.yaml --namespace kvstore
```

### Get istio ingress gateway IP
```sh
kubectl get service istio-ingressgateway --namespace istio-system -o jsonpath='{.status.loadBalancer.ingress[0].ip}'
```

Test
```sh
curl http://$INGRESSGATEWAY_IP/version
```

### Update Deploy istio to use v2-0 subset
```sh
istioctl replace -f k8s/istio/kvstore-istio-2.0.yaml --namespace kvstore
istioctl replace -f k8s/istio/kvstore-istio-destinationrule-2.0.yaml --namespace kvstore
```

Test
```sh
curl http://$INGRESSGATEWAY_IP/version
$ v1
```

Test v2 with feature flag set on the cookie
```sh
curl http://$INGRESSGATEWAY_IP/version --cookie "featureflag=on"
$ v2
```