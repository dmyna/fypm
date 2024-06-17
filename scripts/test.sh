#!/bin/sh
# Docker Future Method
#CONTAINER_NAME="fypm_test"
#sudo -v
#sudo docker cp "$PWD" $CONTAINER_NAME:/app # -> docker
#sudo docker exec -it $CONTAINER_NAME -sh -c "cargo test"

# Local Current Method
if [ "$(dirname)" != "fypm" ]; then
    echo "You only can run this script in the root directory of the repository! Redirecting..."
    cd .. || exit 1
    echo "Verify if it is the root directory of the repository: $PWD (cancel and fix it if not)"
    sleep 5
fi

sh -c "$PWD/scripts/clear_test_dir.sh" || exit 1

cargo test