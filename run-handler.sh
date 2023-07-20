#!/bin/bash
echo 'cmd:exit' | nc localhost 3333 -c
echo 'cmd:exit' | nc localhost 3333 

cd event-handler-rust
sudo cargo run
cd ..
