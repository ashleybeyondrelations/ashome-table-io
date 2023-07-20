#!/bin/bash
LAYOUT=$1

if [[ -z $LAYOUT ]]; then
LAYOUT=ash
fi

make -C virtual-keyboard/wvkbd/ LAYOUT=ash
./virtual-keyboard/wvkbd/wvkbd-$LAYOUT

#kill -s SIGTERM `pidof wvkbd-ash`
#-to toggle : 
#kill -s SIGRTMIN `pidof wvkbd-ash`

