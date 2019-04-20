# Rust KVstore

## Docker

### Build
```
docker build -t sinvalvm/kvstore .
```

### Run
```
docker run -p 0.0.0.0:8000:8000 -d sinvalvm/kvstore
```

### Tag
```
docker tag sinvalvm/kvstore sinvalvm/kvstore:<tag-name>
```

### Dockerhub Push
```
docker push sinvalvm/kvstore:<tag-name>
```

### Pull
```
docker image pull sinvalvm/kvstore:<tag-name>
```

## Kubernetes

### Deploy k8s
```
kubectl create namespace kvstore
kubectl label namespace kvstore istio-injection=enabled
kubectl apply -f k8s/kvstore-deployment-service-1.0.yaml -n kvstore
```

### Deploy istio
```
istioctl create -f k8s/istio/kvstore-istio-1.0.yaml --namespace kvstore
```

### Get istio ingress gateway IP
```
kubectl get service istio-ingressgateway --namespace istio-system -o jsonpath='{.status.loadBalancer.ingress[0].ip}'
```

The application should be running on port 80, access `http://INGRESSGATEWAY_IP`.
