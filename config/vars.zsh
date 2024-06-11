#!/bin/zsh
# -------------------------------------------------- #
#                       FYSM
export FYSM_TEMP=/tmp/fysm
export FYSM_PERMA_LOGS=/var/tmp/logs
export FYSM_PERMA_LOCKS=/var/tmp/locks
export INIT_LOCK=$FYSM_TEMP/fysm_first_init.lock
export TASKSLOCK=$FYSM_PERMA_LOCKS/tasks

# -------------------------------------------------- #
#                       FYPM

export TW_DB="$DATA_DB"/con/data/taskw
export TIMEWARRIORDB="$DATA_DB"/con/data/timew

export WORKTIME=$(cat /var/tmp/current_work_time | sed 's/\s[->].*//g')

export DUE_FILTER="\((due:today or due.after:today) and due.before:tomorrow)"
export TW_FILTER="+ACTIVE or +OVERDUE or +Divisory or ($DUE_FILTER and (WT:Quantify or WT:NonSched))"

# -------------------------------------------------- #
