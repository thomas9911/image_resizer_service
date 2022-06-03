#! /bin/bash
_=$(docker run -d --rm --name minio-server \
    --env MINIO_ROOT_USER="test_key_id" \
    --env MINIO_ROOT_PASSWORD="secret_access_key" \
    --publish '9000:9000' \
    --publish '9001:9001' \
    bitnami/minio:latest)

# docker kill minio-server
