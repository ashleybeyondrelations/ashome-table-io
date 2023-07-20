#!/bin/bash
echo 'cmd:exit' | nc localhost 3333 -c
cd event-handler-rust
sudo cargo run
cd ..
