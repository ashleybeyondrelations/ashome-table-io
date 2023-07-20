#!/bin/bash
#echo 'cmd:exit' | nc localhost 3333 -c
sudo kill -s SIGTERM `pidof event_handler`

cd event-handler-rust
sudo cargo run
cd ..
