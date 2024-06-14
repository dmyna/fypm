#!/bin/sh
sudo -v

DIR="/home/builduser"

build() {
    cmake -DCMAKE_BUILD_TYPE=release .
    make
    sudo make install
}

#       Build Taskwarrior
git clone --recursive -b stable https://github.com/GothenburgBitFactory/taskwarrior.git
cd taskwarrior || exit 1
build

#       Build Timewarrior
git clone --recurse-submodules https://github.com/GothenburgBitFactory/timewarrior
cd timewarrior
build

