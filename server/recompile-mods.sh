#!/bin/sh
set -xe

pushd ../mods
python3 build.py -w world -d ../server/mods
popd

if [[ $1 == '-r' ]] || [[ $1 == '--run' ]]; then
    cargo run
fi
