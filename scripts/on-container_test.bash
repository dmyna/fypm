#!/bin/bash
cp /app/config/taskrc_model /home/fypm/.taskrc
cp -r /app/hooks/* /home/fypm/.taskwarrior/hooks/

chown -R fypm /home/fypm/.taskwarrior
chmod -R 755 /home/fypm/.taskwarrior
chown fypm /home/fypm/.taskrc
chmod 755 /home/fypm/.taskrc

su - fypm -c 'cd /app && CARGO_TERM_COLOR=always /home/fypm/.cargo/bin/cargo test'