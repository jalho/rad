#!/bin/bash

set -e

if test -d ./release.local/; then
    rm -r ./release.local/
fi
mkdir ./release.local/

cd ./rds-sync/
cargo build --release
cd ..
cp ./rds-sync/target/release/rds-sync ./release.local/

cd ./rcon-cli/
cargo build --release
cd ..
cp ./rcon-cli/target/release/rcon-cli ./release.local/

cp -r ./rust-dedicated-server/server/ ./release.local/
cp ./rust-dedicated-server/run-with-carbon.sh ./release.local/
cp ./rust-dedicated-server/rust.service ./release.local/

cd ./release.local/
tar -czf release.tgz *
