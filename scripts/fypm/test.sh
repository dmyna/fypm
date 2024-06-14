#!/bin/sh
CONTAINER_NAME="fypm_test"

sudo -v

sudo docker cp "$PWD" $CONTAINER_NAME:/app # -> docker

# run cargo test in container
sudo docker exec -it $CONTAINER_NAME -sh -c "cargo test"