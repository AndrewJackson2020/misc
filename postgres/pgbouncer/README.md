
```bash
podman build \
    --tag pgbouncer_custom \
    .
```

Run container
```bash
podman run \
    --detach \
    --volume $(pwd)/pg_hba.conf:/home/pgbouncer/config/pg_hba.conf \
    --volume $(pwd)/pgbouncer.ini:/home/pgbouncer/config/pgbouncer.ini \
    --volume $(pwd)/userlist.txt:/home/pgbouncer/config/userlist.txt \
    --network=host \
    pgbouncer_custom:latest
```


debug container
```bash
podman run \
    --volume $(pwd)/pg_hba.conf:/home/pgbouncer/config/pg_hba.conf \
    --volume $(pwd)/pgbouncer.ini:/home/pgbouncer/config/pgbouncer.ini \
    --network=host \
    -it \
    pgbouncer_custom:latest \
    /bin/bash
```
