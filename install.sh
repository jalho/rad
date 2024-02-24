#!/bin/bash

DOWNLOAD_STEAMCMD="https://steamcdn-a.akamaihd.net/client/installer/steamcmd_linux.tar.gz"
DOWNLOAD_MYSTUFF="https://github.com/jalho/DTEK2058-Advanced-Software-Project/releases/download/v0.1.0-alpha/release.tgz"

INITIAL_WORKINGDIR=$(pwd)
echo "DEBUG: INITIAL_WORKINGDIR is $INITIAL_WORKINGDIR"

set -e

useradd --create-home --shell /bin/bash --uid 1000 rust

cd /home/rust/
wget $DOWNLOAD_STEAMCMD
tar -xzf ./steamcmd_linux.tar.gz
rm ./steamcmd_linux.tar.gz

wget $DOWNLOAD_MYSTUFF
tar -xzf ./release.tgz
rm ./release.tgz

chown -R rust:rust /home/rust/
chmod u+x ./run-with-carbon.sh

mv /home/rust/rust.service /etc/systemd/system/
systemctl daemon-reload
systemctl enable rust.service
systemctl start rust.service
