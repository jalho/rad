#!/bin/bash

DOWNLOAD_STEAMCMD="https://steamcdn-a.akamaihd.net/client/installer/steamcmd_linux.tar.gz"
DOWNLOAD_CARBONMOD="https://github.com/CarbonCommunity/Carbon.Core/releases/download/production_build/Carbon.Linux.Release.tar.gz"
DOWNLOAD_MYSTUFF="https://github.com/jalho/DTEK2058-Advanced-Software-Project/releases/download/v0.1.0-alpha/release.tgz"

set -e

useradd --create-home --shell /bin/bash --uid 1000 rust

# SteamCMD dependencies
dpkg --add-architecture i386
apt-get update
apt-get install lib32gcc-s1

mkdir /home/rust/steamcmd/
cd /home/rust/steamcmd/
wget $DOWNLOAD_STEAMCMD
tar -xzf ./steamcmd_linux.tar.gz
rm ./steamcmd_linux.tar.gz

cd /home/rust/
wget $DOWNLOAD_MYSTUFF
tar -xzf ./release.tgz
rm ./release.tgz

wget $DOWNLOAD_CARBONMOD
tar -xzf ./Carbon.Linux.Release.tar.gz
rm ./Carbon.Linux.Release.tar.gz

chown -R rust:rust /home/rust/
chmod u+x ./run-with-carbon.sh

mv /home/rust/rust.service /etc/systemd/system/
systemctl daemon-reload
systemctl enable rust.service
systemctl start rust.service
