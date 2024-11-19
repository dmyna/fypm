FROM archlinux:base-20241117.0.280007

# Create user and directories
RUN groupadd -r fypm \
    && useradd -r -m -g fypm fypm \
    && mkdir /home/fypm/.taskwarrior \
    && mkdir /home/fypm/.taskwarrior/hooks \
    && mkdir /app \
    && chown -R fypm:fypm /app

# ---------- Root ---------- #
USER root

ARG PACMAN_PARALLELDOWNLOADS=5
RUN pacman-key --init \
    && pacman -Sy --noconfirm --noprogressbar --quiet --needed pacman-contrib \
    && sed -i "s/^ParallelDownloads.*/ParallelDownloads = ${PACMAN_PARALLELDOWNLOADS}/g" /etc/pacman.conf

# Install basic tools
RUN pacman -Syu --noconfirm base base-devel git sudo go cmake ncurses

# ---------- Fypm ---------- #
USER fypm

# Install Yay
WORKDIR /home/fypm
RUN git clone https://aur.archlinux.org/yay.git
WORKDIR /home/fypm/yay
RUN makepkg --noconfirm

# ---------- Root ---------- #
USER root

RUN pacman -U --noconfirm /home/fypm/yay/*.pkg.tar.zst \
    && yay -Syyuu --noconfirm

# ---------- Fypm ---------- #
USER fypm

# Install Rust
SHELL ["/bin/bash", "-o", "pipefail", "-c"]
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

# Install Taskwarrior v2.6.2
WORKDIR /home/fypm
RUN curl https://github.com/GothenburgBitFactory/taskwarrior/releases/download/v2.6.2/task-2.6.2.tar.gz \
    && tar -xzf task-2.6.2.tar.gz
WORKDIR /home/fypm/task-2.6.2
RUN cmake -DCMAKE_BUILD_TYPE=release . \
 && make

# ---------- Root ---------- #
USER root

RUN make install \
 && yay -S timew --noconfirm

# ---------- Fypm ---------- #
USER fypm

WORKDIR /app
