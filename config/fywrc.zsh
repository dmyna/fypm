#!/bin/zsh
# Add fypm and fysm to PATH
export PATH=$PATH:$FYPM_DIR/scripts
export PATH=$PATH:$FYSM_DIR/scripts

source $FYW_DIR/config/vars.zsh

source $FYW_DIR/config/aliases.zsh


# Init some fysm scripts if it is the initialization of the system
if test ! -e "$INIT_LOCK" && [ "$DISPLAY" != "" ]; then
    mkdir "$FYSM_TEMP"

    "$FYW_DIR"/config/init_scheduler.zsh

    _task_poly_daemon_ >> /dev/null 2>&1 & disown

    #_verify_eventual_tasks_

    #_verify_active_tasks_ >> /dev/null 2>&1 &
    #_verify_alarm_tasks_ >> "$FYSM_PERMA_LOGS/fypm/_verify_alarm_tasks_$(date +%Y-%m-%d)" 2>&1 &

    #echo "" > "$INIT_LOCK"
fi
