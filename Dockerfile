FROM archlinux:latest

ARG PACMAN_PARALLELDOWNLOADS=5
RUN pacman-key --init \
    && pacman -Sy --noconfirm --noprogressbar --quiet --needed pacman-contrib \
    && sed -i "s/^ParallelDownloads.*/ParallelDownloads = ${PACMAN_PARALLELDOWNLOADS}/g" /etc/pacman.conf

RUN pacman -Syu --noconfirm base base-devel git sudo go

RUN useradd -m builduser

USER builduser
WORKDIR /home/builduser

RUN git clone https://aur.archlinux.org/yay.git
WORKDIR /home/builduser/yay
RUN makepkg --noconfirm

RUN yay -S

USER root

RUN pacman -U --noconfirm /home/builduser/yay/*.pkg.tar.zst

RUN userdel -r builduser