#!/bin/zsh

CURRENT_WT=PreWork
CURRENT_LGT=off

# --------------- WorkTime --------------- #
if [[ $(date +%H%M) -lt 0830 ]]; then
    echo "export DISPLAY=$DISPLAY && $TWBIN/worktime Essentials" | at 08:30
else
    CURRENT_WT=Essentials
fi
if [[ $(date +%H%M) -lt 1000 ]]; then
    echo "export DISPLAY=$DISPLAY && $TWBIN/worktime General" | at 10:00
else
    CURRENT_WT=General
fi
if [[ $(date +%H%M) -lt 1400 ]]; then
    echo "export DISPLAY=$DISPLAY && $TWBIN/worktime Calm" | at 14:00
else
    CURRENT_WT=Calm
fi
if [[ $(date +%H%M) -lt 1800 ]]; then
    echo "export DISPLAY=$DISPLAY && $TWBIN/worktime PreSleep" | at 18:00
else
    CURRENT_WT=PostWork
fi
if [[ $(date +%H%M) -lt 1900 ]]; then
    echo "export DISPLAY=$DISPLAY && $TWBIN/worktime Sleep" | at 19:00
else
    CURRENT_WT=Sleep
fi

sudo -v
worktime $CURRENT_WT