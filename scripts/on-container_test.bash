#!/bin/bash
cp -r /app/hooks/* /home/fypm/.taskwarrior/hooks/

chown -R fypm /home/fypm/.taskwarrior
chmod -R 755 /home/fypm/.taskwarrior
chown fypm /home/fypm/.taskrc
chmod 755 /home/fypm/.taskrc

su - fypm -c 'cd /app && \
    RUST_BACKTRACE=1 \
    CARGO_TERM_COLOR=always \
    CARGO_TARGET_DIR=/home/fypm/.cargo/target \
        /home/fypm/.cargo/bin/cargo test --\
            --test-threads=1'