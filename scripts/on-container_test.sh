#!/bin/sh

cp /app/config/taskrc_model /home/fypm/.taskrc
cp -r /app/hooks/* /home/fypm/.taskwarrior/hooks/
/home/fypm/.cargo/bin/cargo test