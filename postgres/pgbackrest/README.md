# pgbackrest
Container to build and run pgbackrest

Work in progress

# Code Snippets

```bash
podman build \
    --tag pgbackrest \
    .
```

Run container
```bash
podman run \
    --detach \
    --network=host \
    pgbackrest:latest
```


debug container
```bash
podman run \
    --network=host \
    -it \
    pgbackrest:latest \
    /bin/bash
```
