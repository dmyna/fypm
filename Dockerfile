FROM archlinux:latest

# Create user and directories
RUN useradd -m fypm
RUN mkdir /home/fypm/.taskwarrior
RUN mkdir /home/fypm/.taskwarrior/hooks
RUN mkdir /app

# ---------- Root ---------- #
USER root

ARG PACMAN_PARALLELDOWNLOADS=5
RUN pacman-key --init \
    && pacman -Sy --noconfirm --noprogressbar --quiet --needed pacman-contrib \
    && sed -i "s/^ParallelDownloads.*/ParallelDownloads = ${PACMAN_PARALLELDOWNLOADS}/g" /etc/pacman.conf

# Install basic tools
RUN pacman -Syu --noconfirm base base-devel git sudo go wget cmake

# ---------- Fypm ---------- #
USER fypm

# Install Yay
WORKDIR /home/fypm
RUN git clone https://aur.archlinux.org/yay.git
WORKDIR /home/fypm/yay
RUN makepkg --noconfirm

# ---------- Root ---------- #
USER root

RUN pacman -U --noconfirm /home/fypm/yay/*.pkg.tar.zst
RUN yay -Syyuu --noconfirm

# ---------- Fypm ---------- #
USER fypm

# Install Taskwarrior v2.6.2
WORKDIR /home/fypm
RUN wget https://github.com/GothenburgBitFactory/taskwarrior/releases/download/v2.6.2/task-2.6.2.tar.gz
RUN tar -xzf task-2.6.2.tar.gz
WORKDIR /home/fypm/task-2.6.2
RUN cmake -S . -B build -DCMAKE_BUILD_TYPE=RelWithDebInfo
RUN cmake --build build

# Install Rust
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

# Install Timewarrior
RUN yay -S timew --noconfirm

# ---------- Fypm ---------- #
USER fypm

WORKDIR /app
