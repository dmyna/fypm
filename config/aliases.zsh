#!/bin/zsh

PKG="fypm"

# Fypm
alias taadd="$PKG ta-add"
alias taadd-sub="$PKG ta-add-sub"
alias taadd-seq="$PKG ta-add-seq"
alias taadd-brth="$PKG ta-add-brth"
alias taadd-pl="$PKG ta-add-pl"
alias tan="$PKG ta-annotate"
alias tastart="$PKG ta-start"
alias tastop="$PKG ta-stop"
alias tadone="$PKG ta-done"
alias tastat-del="$PKG ta-statistic deleted"
alias tastat-pen="$PKG ta-statistic pending"
alias tals-date="$PKG ta-ls-date"
alias tals-score="$PKG ta-ls-score --"
alias taban="$PKG ta-abandon"

alias taund="$PKG ta-und"
alias tsched="$PKG ta-schedule"
alias tusched="$PKG ta-unschedule"

alias tamas="$PKG ta-ls-mot-and-sub list"
alias ttamas="$PKG ta-ls-mot-and-sub all"

alias taproj="$PKG ta-project"

alias wt="$PKG wt-apply"
alias wt-add="$PKG wt-add"
alias wt-rm="$PKG wt-remove"
alias wt-ls="$PKG wt-ls"


alias tin="$PKG ti-annotate"
alias ticart="$PKG ti-start-correction"
alias ticend="$PKG ti-end-correction"
alias tistart="$PKG ti-start"
alias tiend="$PKG ti-end"
alias tir="$PKG ti-track"
alias tirep="$PKG ti-replace"
alias tils="$PKG ti-ls"

# TaskWarrior
alias talc="task const"
alias talss="_tals alarms"
alias ttals="task +SUBTASK -Ghost list"
alias tagoals="$PKG ta-ls-date GOAL goals --"
alias tagoalss="$PKG ta-ls-date GOAL all-goals --"
alias taduels="$PKG ta-ls-date due list --"
alias taduelss="$PKG ta-ls-date due all --"
alias taals="$PKG ta-ls-date ALARM list --"
alias taalss="$PKG ta-ls-date ALARM all --"
alias taschls="$PKG ta-ls-date scheduled list --"
alias taschlss="$PKG ta-ls-date scheduled all --"

alias taconls="task TYPE:Continuous description.hasnt:Versionamento -Divisory list"
alias taconlss="task TYPE:Continuous description.hasnt:Versionamento -Divisory all"
alias taandl="task rc.report.next.labels=ID,Type,WT,Due,Description,Urg rc.report.next.columns=id,TYPE,WT,due.relative,description,urgency next"

# TimeWarrior
alias tiils='timew summary :ids'

