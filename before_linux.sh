#!/bin/sh
curl https://sh.rustup.rs -sSf | sh -s -- --default-toolchain stable -y
apt-get -y install libopus-dev
