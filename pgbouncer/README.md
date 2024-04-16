
```bash
podman build \
    --tag pgbouncer_custom \
    .
```

Run container
```bash
podman run \
    --volume $(pwd)/pg_hba.conf:/home/pgbouncer/config/pg_hba.conf \
    --volume $(pwd)/pgbouncer.ini:/home/pgbouncer/config/pgbouncer.ini \
    --network=host \
    pgbouncer_custom
```


debug container
```bash
podman run \
    --volume $(pwd)/pg_hba.conf:/home/pgbouncer/config/pg_hba.conf \
    --volume $(pwd)/pgbouncer.ini:/home/pgbouncer/config/pgbouncer.ini \
    --network=host \
    -it \
    pgbouncer_custom \
    /bin/bash
```
