#!/bin/bash
LAYOUT=$1

if [[ -z $LAYOUT ]]; then
LAYOUT=ash
fi

make -C wvkbd/ LAYOUT=ash
./wvkbd/wvkbd-$LAYOUT

#kill -s SIGTERM `pidof wvkbd-ash`
#-to toggle : 
#kill -s SIGRTMIN `pidof wvkbd-ash`

