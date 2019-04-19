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

### Deploy
