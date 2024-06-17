#!/bin/sh

# This script assumes that you are in the root directory of the repository!

TEST_DIR="$PWD/__test_dir__"

if [ ! -d "$TEST_DIR" ]; then
    mkdir "$TEST_DIR"
else
    rm -rf "${TEST_DIR:?}/*" || exit 1
    cp "$PWD/config/taskrc_model" "$TEST_DIR/.taskrc"
    mkdir "$TEST_DIR/task"
    mkdir "$TEST_DIR/timew"
fi

